mod isp;
pub use isp::{BootInfo, EraseFlash, GetBootInfo, IspCommand, IspError, WriteFlash};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use object::{Object, ObjectSection, SectionFlags};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

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
