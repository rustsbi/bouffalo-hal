use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const HEAD_LENGTH: u64 = 0x160;
const HEAD_MAGIC: u32 = 0x42464e50;
const FLASH_MAGIC: u32 = 0x46434647;
const CLK_MAGIC: u32 = 0x50434647;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Wrong magic number")]
    MagicNumber { wrong_magic: u32 },
    #[error("File is too short to include an image header, should include {HEAD_LENGTH} but only {wrong_length}")]
    HeadLength { wrong_length: u64 },
    #[error("Wrong flash config magic")]
    FlashConfigMagic,
    #[error("Wrong clock config magic")]
    ClockConfigMagic,
    #[error("Image offset overflow, offset {wrong_image_offset} and length {wrong_image_length} expected, but file length is {file_length}")]
    ImageOffsetOverflow {
        file_length: u64,
        wrong_image_offset: u32,
        wrong_image_length: u32,
    },
    #[error("Wrong sha256 checksum")]
    Sha256Checksum,
}

pub type Result = core::result::Result<(), Error>;

pub fn process(f: &mut File) -> Result {
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
        return Err(Error::FlashConfigMagic);
    }

    f.seek(SeekFrom::Start(0x64))?;
    let clk_magic = f.read_u32::<BigEndian>()?;
    if clk_magic != CLK_MAGIC {
        return Err(Error::ClockConfigMagic);
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

    //read hash values from file
    f.seek(SeekFrom::Start(0x90))?;
    let mut hash = vec![0; 32];
    let _ = f.read(&mut hash)?;

    //calculate hash
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

    let hash2 = hasher.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&hash2);
    let vec1 = Vec::from(array);

    if vec1 != hash {
        let vec2: Vec<u8> = vec![0xdeadbeef, 0, 0, 0, 0, 0, 0, 0]
            .iter()
            .map(|x: &u32| *x as u8)
            .collect();
        let vec3: Vec<u8> = vec![0xdeadbeef; 8].iter().map(|x: &u32| *x as u8).collect();
        if hash != vec2 && hash != vec3 {
            return Err(Error::Sha256Checksum);
        }
    }

    let _ = f.read(&mut buffer)?;
    println!("image content: {:?}", buffer);

    Ok(())
}
