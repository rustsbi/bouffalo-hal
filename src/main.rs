use blri::Error;
use clap::Parser;
use std::fs::File;

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
    let mut f = File::open(args.input).expect("open file");

    match blri::process(&mut f) {
		Ok(_) => {println!("success");},
		Err(e) => match e {
			Error::MagicNumber { wrong_magic } => println!("error: incorrect magic number {wrong_magic}!"),
            Error::HeadLength { wrong_length } =>
                println!("File is too short to include an image header, it only includes {wrong_length} bytes"),
            Error::FlashConfigMagic => println!("error: incorrect flash config magic!"),
            Error::ClockConfigMagic => println!("error: incorrect clock config magic!"),
			Error::ImageOffsetOverflow { file_length, wrong_image_offset, wrong_image_length } =>
                println!("error: file length is only {}, but offset is {} and image length is {}", file_length, wrong_image_offset, wrong_image_length),
            Error::Sha256Checksum => println!("error: Sha256 verification failed!"),
            Error::Io(source) => println!("error: io error! {:?}", source)
		}
	}

    // println!("Input file name: {}!", args.input);
    // println!("Output file name: {:?}!", args.output);
}
