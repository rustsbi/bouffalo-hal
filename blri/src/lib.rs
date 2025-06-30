mod isp;
use bouffalo_rt::soc::bl808::HalBootheader;
use bouffalo_rt::{BFLB_BOOT2_HEADER_MAGIC, BasicConfigFlags};
use clap::Args;
pub use isp::{BootInfo, DeviceReset, EraseFlash, GetBootInfo, IspCommand, IspError, WriteFlash};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use object::{Object, ObjectSection, SectionFlags};
use sha2::{Digest, Sha256};
use std::cmp::max;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

const HEAD_LENGTH: u64 = 0x160;
const HEAD_MAGIC: u32 = 0x42464e50;
const FLASH_MAGIC: u32 = 0x46434647;
const CLOCK_MAGIC: u32 = 0x50434647;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Wrong magic number")]
    MagicNumber { wrong_magic: u32 },
    #[error(
        "File is too short to include an image header, should include {HEAD_LENGTH} but only {wrong_length}"
    )]
    HeadLength { wrong_length: u64 },
    #[error("Wrong flash config magic")]
    FlashConfigMagic { wrong_magic: u32 },
    #[error("Wrong clock config magic")]
    ClockConfigMagic { wrong_magic: u32 },
    #[error(
        "Image offset overflow, offset {wrong_image_offset} and length {wrong_image_length} expected, but file length is {file_length}"
    )]
    ImageOffsetOverflow {
        file_length: u64,
        wrong_image_offset: u32,
        wrong_image_length: u32,
    },
    #[error("Wrong sha256 checksum")]
    Sha256Checksum { wrong_checksum: Vec<u8> },
}

/// Process operations.
pub struct Operations {
    /// Refill hash value of image body into header, or None if not needed.
    ///
    /// Should include 32 bytes for sha256 algorithm.
    pub refill_hash: Option<Vec<u8>>,
    /// Refill CRC32 value of header, None if not needed.
    pub refill_header_crc: Option<u32>,
}

pub type Result<T> = core::result::Result<T, Error>;

/// Check source file without modifying, returning suggested operations.
///
/// File `f` should be readable, but not writable.
pub fn check(f: &mut File) -> Result<Operations> {
    let file_length = f.metadata()?.len();

    f.seek(SeekFrom::Start(0x00))?;
    let head_magic = f.read_u32::<BigEndian>()?;
    if head_magic != HEAD_MAGIC {
        return Err(Error::MagicNumber {
            wrong_magic: head_magic,
        });
    }

    if file_length < HEAD_LENGTH {
        return Err(Error::HeadLength {
            wrong_length: file_length,
        });
    }

    f.seek(SeekFrom::Start(0x08))?;
    let flash_magic = f.read_u32::<BigEndian>()?;
    if flash_magic != FLASH_MAGIC {
        return Err(Error::FlashConfigMagic {
            wrong_magic: flash_magic,
        });
    }

    f.seek(SeekFrom::Start(0x64))?;
    let clock_magic = f.read_u32::<BigEndian>()?;
    if clock_magic != CLOCK_MAGIC {
        return Err(Error::ClockConfigMagic {
            wrong_magic: clock_magic,
        });
    }

    f.seek(SeekFrom::Start(0x84))?;
    let group_image_offset = f.read_u32::<LittleEndian>()?;

    f.seek(SeekFrom::Start(0x8C))?;
    let image_body_length = f.read_u32::<LittleEndian>()?;

    if group_image_offset as u64 + image_body_length as u64 > file_length {
        return Err(Error::ImageOffsetOverflow {
            file_length,
            wrong_image_offset: group_image_offset,
            wrong_image_length: image_body_length,
        });
    }

    // read hash values from file
    f.seek(SeekFrom::Start(0x90))?;
    let mut actual_hash = vec![0; 32];
    f.read_exact(&mut actual_hash)?;

    // calculate hash
    f.seek(SeekFrom::Start(group_image_offset as u64))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; image_body_length as usize];
    loop {
        let length_read = f.read(&mut buffer)?;
        if length_read == 0 {
            break;
        }
        hasher.update(&buffer[..length_read]);
    }

    let calculated_hash = &hasher.finalize()[..];

    let refill_hash_operation = if calculated_hash != actual_hash {
        let mut candidate_hash_1 = vec![0u8; 32];
        candidate_hash_1[..4].copy_from_slice(&[0xef, 0xbe, 0xad, 0xde]);
        let mut candidate_hash_2 = vec![0u8; 32];
        for i in 0..8 {
            candidate_hash_2[4 * i..4 * (i + 1)].copy_from_slice(&[0xef, 0xbe, 0xad, 0xde]);
        }
        if actual_hash != candidate_hash_1 && actual_hash != candidate_hash_2 {
            return Err(Error::Sha256Checksum {
                wrong_checksum: actual_hash,
            });
        }
        Some(Vec::from(calculated_hash))
    } else {
        // source image hash is correct, do not need to fill
        None
    };

    f.seek(SeekFrom::Start(0x00))?;
    let mut buf = vec![0u8; 0x15C];
    f.read_exact(&mut buf)?;
    if let Some(ref new_hash) = refill_hash_operation {
        buf[0x90..0xB0].copy_from_slice(new_hash);
    }
    let calculated_header_crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&buf);

    f.seek(SeekFrom::Start(0x15C))?;
    let read_head_crc = f.read_u32::<LittleEndian>()?;

    let refill_header_crc_operation =
        if read_head_crc != calculated_header_crc || refill_hash_operation.is_some() {
            Some(calculated_header_crc)
        } else {
            None
        };

    Ok(Operations {
        refill_hash: refill_hash_operation,
        refill_header_crc: refill_header_crc_operation,
    })
}

/// Process target file from operations.
pub fn process(f: &mut File, ops: &Operations) -> Result<()> {
    if let Some(hash_to_fill) = &ops.refill_hash {
        f.seek(SeekFrom::Start(0x90))?;
        f.write(&hash_to_fill[..32])?;
    }
    if let Some(header_crc_to_fill) = &ops.refill_header_crc {
        f.seek(SeekFrom::Start(0x15C))?;
        f.write_u32::<LittleEndian>(*header_crc_to_fill)?;
    }
    Ok(())
}

// The following functions are for elf2bin module
// Most of the code is adapted from `https://github.com/llvm/llvm-project/tree/main/llvm/lib/ObjCopy/ELF`

/// Main logic for converting ELF to binary, adapted from LLVM's objcopy
///
/// Ref: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObjcopy.cpp  `Error
/// objcopy::elf::executeObjcopyOnBinary()` method
pub fn elf_to_bin_bytes(elf_data: &[u8]) -> Result<Vec<u8>> {
    // Parse the ELF file
    let elf_file = object::File::parse(elf_data)
        .map_err(|e| Error::Io(io::Error::new(io::ErrorKind::Other, e)))?;

    // Get loadable sections
    let mut sections = get_loadable_sections(&elf_file);
    // Sort sections by their offset in the file
    sort_sections_with_offset(&mut sections);

    // Log section information
    log_section_info(&sections);

    // Create final binary output
    let output_data = process_sections(sections)?;

    Ok(output_data)
}

/// Wrapper function for converting ELF to binary, takes input and output file paths
pub fn elf_to_bin(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<()> {
    // Read the ELF file
    let elf_data = fs::read(input_path)?;

    // Convert ELF to binary
    let bin_data = elf_to_bin_bytes(&elf_data)?;

    // Write the binary data to the output file
    fs::write(output_path, bin_data)?;

    Ok(())
}

// The following functions are helpers for elf2bin module

/// Log section information using `println`
fn log_section_info(sections: &[object::Section]) {
    println!("Found {} loadable sections", sections.len());

    for section in sections {
        println!(
            "Section: {} at address 0x{:x} with size 0x{:x}",
            section.name().unwrap_or("<unnamed>"),
            section.address(),
            section.size()
        );
    }
}

/// Get loadable sections from the ELF file
///
/// Loadable sections are those with the ALLOC section header flag set
///
/// Ref: https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObject.cpp `Error BinaryWriter::finalize()` method
fn get_loadable_sections<'a>(elf_file: &'a object::File) -> Vec<object::Section<'a, 'a>> {
    // Find sections with ALLOC flag
    let mut sections: Vec<_> = elf_file
        .sections()
        .filter(|s| {
            // Check if section has ALLOC flag set (should be loaded into memory)
            match s.flags() {
                SectionFlags::Elf { sh_flags } => (sh_flags & object::elf::SHF_ALLOC as u64) != 0,
                _ => false, // Other formats don't apply for ELF conversion
            }
        })
        .collect();

    // Sort sections by address
    sections.sort_by_key(|s| s.address());

    sections
}

/// Get the offset of a section using the `compressed_file_range` method,
/// panic if this method fails.
fn get_section_offset(section: &object::Section) -> u64 {
    section
        .compressed_file_range()
        .expect("Section file range not found!")
        .offset
}

/// Sort sections by their offset in the file
///
/// Ref:
/// https://github.com/llvm/llvm-project/blob/main/llvm/lib/ObjCopy/ELF/ELFObject.cpp
/// `Error BinaryWriter::write()`
fn sort_sections_with_offset(sections: &mut Vec<object::Section>) {
    sections.sort_by_key(|s| get_section_offset(s));
}

/// Process sections and write them to the output binary, the input sections
/// must be sorted properly by their offset in the file
fn process_sections(sections: Vec<object::Section>) -> Result<Vec<u8>> {
    let mut output_data = Vec::new();

    for section in sections {
        let addr = section.address();
        let size = section.size();
        let name = section.name().unwrap_or("<unnamed>");

        println!(
            "Writing section: {} at address 0x{:x} with size 0x{:x}",
            name, addr, size
        );

        if size == 0 {
            continue;
        }

        write_section_data(&mut output_data, &section)?;
    }

    Ok(output_data)
}

/// Write section data to the output binary
fn write_section_data(output: &mut Vec<u8>, section: &object::Section) -> Result<()> {
    // Handle regular sections and NOBITS sections differently
    if let Ok(data) = section.data() {
        // Regular section - copy the data
        output.write_all(data)?;
    } else {
        // NOBITS section (like .bss) - write zeros
        let zeros = vec![0u8; section.size() as usize];
        println!(
            "Section {} is NOBITS, writing zeros of size 0x{:x}",
            section.name().unwrap_or("<unnamed>"),
            section.size()
        );
        output.write_all(&zeros)?;
    }

    Ok(())
}

/// Parse a hexadecimal string into a u32 value.
fn parse_hex_u32(s: &str) -> core::result::Result<u32, ParseIntError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u32::from_str_radix(s, 16)
}

/// Represents an image to be flashed, including its path and load address (in hex).
#[derive(Args, Debug, Clone)]
pub struct ImageToFuse {
    /// 镜像文件路径
    #[arg(long, value_name = "FILE")]
    pub path: PathBuf,

    /// 镜像在闪存中的加载地址 (十六进制)
    #[arg(long, value_name = "ADDRESS", value_parser = parse_hex_u32)]
    pub addr: u32,
}

fn all_fields_equal<'a, T: 'a, F, V>(iter: impl Iterator<Item = &'a T>, f: F) -> bool
where
    F: Fn(&T) -> &V,
    V: PartialEq + 'a,
{
    let mut iter = iter.map(f);
    if let Some(first) = iter.next() {
        iter.all(|v| v == first)
    } else {
        true // Empty iterator, consider all fields equal
    }
}

/// Fuse multiple images into a single image.
/// Currently only supports fusing a M0 image and a D0 image into a fused image.
// todo: Figure out whether the parameters can be passed as a Vec
pub fn fuse_image_header(
    m0_image: Option<ImageToFuse>,
    d0_image: Option<ImageToFuse>,
    lp_image: Option<ImageToFuse>,
) -> HalBootheader {
    // todo: Validate the images before fusing

    if lp_image.is_some() {
        todo!("lp_image is not supported yet, please use m0_image and d0_image only.");
    }
    let images_to_fuse = vec![m0_image.clone(), d0_image.clone(), lp_image.clone()]
        .iter()
        .filter_map(|img| img.clone())
        .collect::<Vec<_>>();
    if images_to_fuse.len() < 2 {
        panic!("error: at least two images are required to fuse.");
    }
    let loaded_image_m0 = m0_image.map(|img| {
        let mut file = File::open(&img.path).expect("open M0 image file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("read M0 image file");

        (HalBootheader::from_bytes(&data).unwrap(), img.addr)
    });

    let loaded_image_d0 = d0_image.map(|img| {
        let mut file = File::open(&img.path).expect("open D0 image file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("read D0 image file");

        (HalBootheader::from_bytes(&data).unwrap(), img.addr)
    });

    let loaded_image_lp = lp_image.map(|img| {
        let mut file = File::open(&img.path).expect("open LP image file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("read LP image file");

        (HalBootheader::from_bytes(&data).unwrap(), img.addr)
    });

    let loaded_images: Vec<_> = images_to_fuse
        .iter()
        .map(|img| {
            let mut file = File::open(&img.path).expect("open image file");
            let mut data = Vec::new();
            file.read_to_end(&mut data).expect("read image file");

            (HalBootheader::from_bytes(&data).unwrap(), img.addr)
        })
        .collect();

    macro_rules! validate_all {
    // Macro arguments: the collection to iterate, the field to check, the expected value,
    // and a code block to execute on failure.
    ($collection:expr, $field:ident, $expected_value:expr, on_fail: {$($fail_action:tt)*}) => {
        // Use .iter().all() to check every element in the collection
        let all_match = $collection.iter().all(|(header, _)| header.$field == $expected_value);

        // If the validation fails (all_match is false)
        if !all_match {
            // Execute the code provided in the on_fail block
            $($fail_action)*
        }
    };}

    macro_rules! validate_all_same {
    ($collection:expr, $field:ident, on_fail: {$($fail_action:tt)*}) => {
        let all_same = if $collection.is_empty() {
            // If the collection is empty, consider all fields equal
            true
        } else{
            // Use .iter().map() to extract the field and check if all values are the same
            all_fields_equal($collection.iter(), |(header, _)| &header.$field)
        };
        if !all_same {
            $($fail_action)*
        }
    }}
    // Example of checks, currently not enouth
    // TODO: Add more checks for the images

    // Check the 'magic' field
    validate_all!(
        loaded_images,
        magic,
        BFLB_BOOT2_HEADER_MAGIC,
        on_fail: {
            panic!("Not all images have the same magic number, cannot fuse.");
        }
    );

    let fused_image_header_magic = BFLB_BOOT2_HEADER_MAGIC;

    const DEFAULT_REVISION: u32 = 0x01;
    // Check the 'revision' field
    validate_all!(
        loaded_images,
        revision,
        DEFAULT_REVISION, // Or use the DEFAULT_REVISION constant
        on_fail: {
            panic!("Not all images have the same revision, using default revision 0x01.");
        }
    );
    let fused_image_header_revision = DEFAULT_REVISION;

    // From current evidence, the flash configuration is the same for all three tested images.
    validate_all_same!(
        loaded_images,
        flash_cfg,
        on_fail: {
            panic!("Not all images have the same flash ID, cannot fuse NOW.");
        }
    );
    let fused_image_header_flash_cfg = loaded_images[0].0.flash_cfg.clone();

    // From current evidence, the clock configuration is the same for all three
    // tested images.
    validate_all_same!(
        loaded_images,
        clk_cfg,
        on_fail: {
            panic!("Not all images have the same clock configuration, cannot fuse NOW.");
        }
    );

    let fused_image_header_clk_cfg = loaded_images[0].0.clk_cfg.clone();

    // For BasicConfig, it still follow's some default pattern, but some fields
    // are overwritten after image fusion.

    let fused_image_header_basic_cfg = {
        let mut basic_cfg = loaded_images[0].0.basic_cfg.clone();
        let mut basic_cfg_flags = BasicConfigFlags::from_u32(basic_cfg.flag);
        // See `chips/bl808/img_create_iot/efuse_bootheader_cfg.ini` in BouffaloLabDevCube-v1.9.0
        basic_cfg_flags.cmds_wrap_mode = 1;
        basic_cfg_flags.cmds_wrap_len = 9;
        basic_cfg_flags.update_raw();

        // Set the magic number for the fused image
        // Set the flags to 0, as we don't need any special flags for the fused image
        basic_cfg.flag = basic_cfg_flags.raw;

        // See `chips/bl808/img_create_iot/efuse_bootheader_cfg.ini` in BouffaloLabDevCube-v1.9.0
        basic_cfg.group_image_offset = 0x2000;

        // In `chips/bl808/img_create_iot/efuse_bootheader_cfg.ini` in
        // BouffaloLabDevCube-v1.9.0, the `img_len_cnt` is set to 0x100.

        // 216+2384 = 4384, just guess the minimum image length is 2000 in fused
        // image.
        // TODO: figure out the correct image length for the fused image.
        basic_cfg.img_len_cnt = loaded_images
            .iter()
            .map(|(header, _)| max(2000, header.basic_cfg.img_len_cnt))
            .sum();

        //TODO: figure out the correct hash for the fused image in bosic_cfg
        // The hash is realted to the whole image, not just the header, so we
        // ignore it for now.

        basic_cfg
    };

    let mut fused_image_cpu_configs = loaded_images[0].0.cpu_cfg.clone(); // Start with the first image's CPU configs

    if let Some((ref m0_header, boot_entry)) = loaded_image_m0 {
        if m0_header.cpu_cfg[0].config_enable != 0 {
            fused_image_cpu_configs[0] = m0_header.cpu_cfg[0].clone();
            fused_image_cpu_configs[0].boot_entry = boot_entry; // Set the boot entry for M0
            // From current evidence, the M0 image address offset is directly
            // set to 0x1000
            // See `chips/bl808/img_create_iot/efuse_bootheader_cfg.ini` in
            // BouffaloLabDevCube-v1.9.0
            //TODO: figure out the correct image address offset for the M0 image
            //CPU config.

            fused_image_cpu_configs[0].image_address_offset = 0x1000;
        }
    }
    if let Some((ref d0_header, boot_entry)) = loaded_image_d0 {
        if d0_header.cpu_cfg[1].config_enable != 0 {
            fused_image_cpu_configs[1] = d0_header.cpu_cfg[1].clone();
            fused_image_cpu_configs[1].boot_entry = boot_entry; // Set the boot entry for D0
        }
    }
    if let Some((ref lp_header, boot_entry)) = loaded_image_lp {
        if lp_header.cpu_cfg[2].config_enable != 0 {
            fused_image_cpu_configs[2] = lp_header.cpu_cfg[2].clone();
            fused_image_cpu_configs[2].boot_entry = boot_entry; // Set the boot entry for LP
        }
    }

    // All cpu config that are not enabled shoule be default value, so ingore
    // for now
    //TODO: figure out these default values and fill fused_image_cpu_configs
    //with these default values.

    // TODO: The following 4 fields have no obvious clues
    validate_all!(
        loaded_images,
        boot2_pt_table_0,
        0, // The partition table is not used in the fused image, so set it
        on_fail: {
            println!("Not all images have the same boot2_pt_table_0, using 0
    as default.");}
    );
    let fused_image_header_boot2_pt_table_0 = 0;

    validate_all!(
        loaded_images,
        boot2_pt_table_1,
        0, // The partition table is not used in the fused image, so set it
        on_fail: {
            println!("Not all images have the same boot2_pt_table_1, using 0
    as default.");}
    );
    let fused_image_header_boot2_pt_table_1 = 0;

    // TODO: figure out why and how
    let fused_image_header_basic_flash_cfg_table_addr = 0x160;

    validate_all!(
        loaded_images,
        flash_cfg_table_len,
        0, // The basic flash config table length is not used in the fused image, so set it
        on_fail: {
            println!("Not all images have the same basic_flash_cfg_table_len, using 0
    as default.");}
    );
    let fused_image_header_basic_flash_cfg_table_len = 0;

    // Seems all default values
    let fused_image_header_patch_on_read = loaded_images[0].0.patch_on_read.clone();
    let fused_image_header_patch_on_jump = loaded_images[0].0.patch_on_jump.clone();
    let fused_image_header_reserved = loaded_images[0].0._reserved.clone();
    let mut fused_image = HalBootheader {
        magic: fused_image_header_magic,
        revision: fused_image_header_revision,
        flash_cfg: fused_image_header_flash_cfg,
        clk_cfg: fused_image_header_clk_cfg,
        basic_cfg: fused_image_header_basic_cfg,
        cpu_cfg: fused_image_cpu_configs,
        boot2_pt_table_0: fused_image_header_boot2_pt_table_0,
        boot2_pt_table_1: fused_image_header_boot2_pt_table_1,
        flash_cfg_table_addr: fused_image_header_basic_flash_cfg_table_addr,
        flash_cfg_table_len: fused_image_header_basic_flash_cfg_table_len,
        patch_on_read: fused_image_header_patch_on_read,
        patch_on_jump: fused_image_header_patch_on_jump,
        _reserved: fused_image_header_reserved,
        crc32: 0,
    };
    fused_image.update_crc32();
    fused_image
}
