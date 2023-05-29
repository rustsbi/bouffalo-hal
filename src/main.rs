use blri::Error;
use clap::Parser;
use std::fs::{self, File};

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

fn main() {
    let args = Args::parse();
    let mut f_in = File::open(&args.input).expect("open input file");

    let ops = match blri::check(&mut f_in) {
        Ok(ops) => ops,
        Err(e) => match e {
            Error::MagicNumber { wrong_magic } => {
                println!("error: incorrect magic number {wrong_magic}!");
                return;
            }
            Error::HeadLength { wrong_length } => {
                println!("File is too short to include an image header, it only includes {wrong_length} bytes");
                return;
            }
            Error::FlashConfigMagic => {
                println!("error: incorrect flash config magic!");
                return;
            }
            Error::ClockConfigMagic => {
                println!("error: incorrect clock config magic!");
                return;
            }
            Error::ImageOffsetOverflow {
                file_length,
                wrong_image_offset,
                wrong_image_length,
            } => {
                println!(
                    "error: file length is only {}, but offset is {} and image length is {}",
                    file_length, wrong_image_offset, wrong_image_length
                );
                return;
            }
            Error::Sha256Checksum => {
                println!("error: wrong sha256 verification.");
                return;
            }
            Error::Io(source) => {
                println!("error: io error! {:?}", source);
                return;
            }
        },
    };

    let output = args.output.clone().unwrap_or(args.input.clone());

    if output != args.input {
        fs::copy(&args.input, &output).expect("copy input to output");
    }

    // release input file
    drop(f_in);

    // open output file as writeable
    let mut f_out = File::options()
        .write(true)
        .create(true)
        .open(output)
        .expect("open output file");

    blri::process(&mut f_out, &ops).expect("process file");
}
