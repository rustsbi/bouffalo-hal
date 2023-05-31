use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

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
    #[error("File is too short to include an image header, should include {HEAD_LENGTH} but only {wrong_length}")]
    HeadLength { wrong_length: u64 },
    #[error("Wrong flash config magic")]
    FlashConfigMagic { wrong_magic: u32 },
    #[error("Wrong clock config magic")]
    ClockConfigMagic { wrong_magic: u32 },
    #[error("Image offset overflow, offset {wrong_image_offset} and length {wrong_image_length} expected, but file length is {file_length}")]
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
    let clk_magic = f.read_u32::<BigEndian>()?;
    if clk_magic != CLOCK_MAGIC {
        return Err(Error::ClockConfigMagic {
            wrong_magic: clk_magic,
        });
    }

    f.seek(SeekFrom::Start(0x84))?;
    let group_image_offset = f.read_u32::<LittleEndian>()?;

    f.seek(SeekFrom::Start(0x8C))?;
    let img_len_cnt = f.read_u32::<LittleEndian>()?;

    if group_image_offset as u64 + img_len_cnt as u64 > file_length {
        return Err(Error::ImageOffsetOverflow {
            file_length,
            wrong_image_offset: group_image_offset,
            wrong_image_length: img_len_cnt,
        });
    }

    // read hash values from file
    f.seek(SeekFrom::Start(0x90))?;
    let mut read_hash = vec![0; 32];
    f.read_exact(&mut read_hash)?;

    // calculate hash
    f.seek(SeekFrom::Start(group_image_offset as u64))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; img_len_cnt as usize];
    loop {
        let n = f.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let calculated_hash = &hasher.finalize()[..];

    let refill_hash = if calculated_hash != read_hash {
        let mut vec2 = vec![0u8; 32];
        vec2[..4].copy_from_slice(&[0xef, 0xbe, 0xad, 0xde]);
        let mut vec3 = vec![0u8; 32];
        for i in 0..8 {
            vec3[4 * i..4 * (i + 1)].copy_from_slice(&[0xef, 0xbe, 0xad, 0xde]);
        }
        if read_hash != vec2 && read_hash != vec3 {
            return Err(Error::Sha256Checksum {
                wrong_checksum: read_hash,
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
    let calculated_header_crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&buf);

    f.seek(SeekFrom::Start(0x15C))?;
    let read_head_crc = f.read_u32::<LittleEndian>()?;

    let refill_header_crc = if read_head_crc != calculated_header_crc || refill_hash.is_some() {
        Some(calculated_header_crc)
    } else {
        None
    };

    Ok(Operations {
        refill_hash,
        refill_header_crc,
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
