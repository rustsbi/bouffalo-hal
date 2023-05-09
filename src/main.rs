use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// Bouffalo ROM image helper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input ROM image filename
    input: String,
    /// Write output to <filename>
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<String>,
}

const HEAD_MAGIC: u32 = 0x42464e50;
const FLASH_MAGIC: u32 = 0x46434647;
const CLK_MAGIC: u32 = 0x50434647;

fn main() {
    let args = Args::parse();
    let mut f = File::open(args.input).unwrap();

    f.seek(SeekFrom::Start(0x00)).unwrap();
    let head_magic = f.read_u32::<BigEndian>().unwrap();

    if head_magic != HEAD_MAGIC {
        println!("error: incorrect magic number!");
        return;
    }

    f.seek(SeekFrom::Start(0x08)).unwrap();
    let flash_magic = f.read_u32::<BigEndian>().unwrap();

    if flash_magic != FLASH_MAGIC {
        println!("error: incorrect flash config magic!");
        return;
    }

    f.seek(SeekFrom::Start(0x64)).unwrap();
    let clk_magic = f.read_u32::<BigEndian>().unwrap();

    if clk_magic != CLK_MAGIC {
        println!("error: incorrect clock config magic!");
        return;
    }

    f.seek(SeekFrom::Start(0x84)).unwrap();
    let group_magic_offset = f.read_u32::<LittleEndian>().unwrap();

    f.seek(SeekFrom::Start(0x8C)).unwrap();
    let img_len_cnt = f.read_u32::<LittleEndian>().unwrap();

    f.seek(SeekFrom::Start(group_magic_offset as u64)).unwrap();
    let mut buffer = vec![0; img_len_cnt as usize];

    let _ = f.read(&mut buffer).unwrap();

    println!("image content: {:?}", buffer);

    // println!("Input file name: {}!", args.input);
    // println!("Output file name: {:?}!", args.output);
}
