use blri::Error;
use blri::elf_to_bin;
use clap::{Args, Parser, Subcommand};
use inquire::Select;
use serialport::SerialPort;
use std::{
    cmp::min,
    fs::{self, File},
    thread::sleep,
    time::Duration,
};

#[derive(Parser)]
#[clap(name = "blri")]
#[clap(about = "Bouffalo ROM image helper")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply patches to a image, such as fixing CRC32 checksums and other necessary corrections.
    Patch(Patch),
    /// Flash the image to a device.
    Flash(Flash),
    /// Convert ELF file to binary file.
    Elf2bin(Elf2Bin),
}

#[derive(Args)]
struct Patch {
    /// The path to the image file that needs to be patched.
    input_file: String,
    /// The path to save the patched image file. If not provided, the input file will be overwritten.
    output_file: Option<String>,
}

#[derive(Args)]
struct Flash {
    /// The path to the image file that needs to be flashed.
    image: String,
    /// The serial port to use for flashing. If not provided, a list of available ports will be shown.
    #[clap(short, long)]
    port: Option<String>,
}

#[derive(Args)]
struct Elf2Bin {
    /// The path to the input ELF file.
    input_file: String,
    /// The path to save the output binary file. If not provided, uses the input filename with .bin extension.
    #[clap(short, long)]
    output_file: Option<String>,
    /// Whether to patch the output binary automatically.
    #[clap(short, long)]
    patch: bool,
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Patch(patch) => {
            let input_file = patch.input_file.clone();
            let output_file = patch.output_file.clone().unwrap_or(input_file.clone());
            patch_image(input_file, output_file);
        }
        Commands::Flash(flash) => {
            let port = match &flash.port {
                Some(port) => port.clone(),
                None => {
                    let ports = serialport::available_ports().expect("list serial ports");
                    let mut port_names: Vec<String> =
                        ports.iter().map(|p| p.port_name.clone()).collect();
                    port_names.sort();
                    Select::new("Select a serial port", port_names)
                        .prompt()
                        .expect("select serial port")
                }
            };
            flash_image(flash.image.clone(), port);
        }
        Commands::Elf2bin(elf2bin) => {
            let input_file = elf2bin.input_file.clone();
            // if output_file is not provided, use input filename with .bin extension
            let output_file = elf2bin.output_file.clone().unwrap_or_else(|| {
                let mut output = input_file.clone();
                output.push_str(".bin");
                output
            });
            elf_to_bin(&input_file, &output_file).expect("Unable to convert ELF to BIN");
            if elf2bin.patch {
                // TODO: add a inner `patch_image` for bytes to patch the output
                // TODO: binary before saving into file system.
                patch_image(output_file.clone(), output_file);
            }
        }
    }
}

fn patch_image(input_file: String, output_file: String) {
    let mut f_in = File::open(&input_file).expect("open input file");

    let ops = match blri::check(&mut f_in) {
        Ok(ops) => ops,
        Err(e) => match e {
            Error::MagicNumber { wrong_magic } => {
                println!("error: incorrect magic number 0x{wrong_magic:08x}!");
                return;
            }
            Error::HeadLength { wrong_length } => {
                println!(
                    "File is too short to include an image header, it only includes {wrong_length} bytes"
                );
                return;
            }
            Error::FlashConfigMagic { wrong_magic } => {
                println!("error: incorrect flash config magic 0x{wrong_magic:08x}!");
                return;
            }
            Error::ClockConfigMagic { wrong_magic } => {
                println!("error: incorrect clock config magic 0x{wrong_magic:08x}!");
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
            Error::Sha256Checksum { wrong_checksum } => {
                let mut wrong_checksum_hex = String::new();
                for i in wrong_checksum {
                    wrong_checksum_hex.push_str(&format!("{:02x}", i));
                }
                println!("error: wrong sha256 verification: {}.", wrong_checksum_hex);
                return;
            }
            Error::Io(source) => {
                println!("error: io error! {:?}", source);
                return;
            }
        },
    };

    if output_file != input_file {
        fs::copy(&input_file, &output_file).expect("copy input to output");
    }

    // release input file
    drop(f_in);

    // open output file as writeable
    let mut f_out = File::options()
        .write(true)
        .create(true)
        .open(&output_file)
        .expect("open output file");

    blri::process(&mut f_out, &ops).expect("process file");
    println!("patched image saved to {}", output_file);
}

fn flash_image(image: String, port: String) {
    const BAUDRATE: u32 = 2000000;
    const USB_INIT: &[u8] = b"BOUFFALOLAB5555RESET\0\x01";
    const HANDSHAKE: &[u8] = &[
        0x50, 0x00, 0x08, 0x00, 0x38, 0xF0, 0x00, 0x20, 0x00, 0x00, 0x00, 0x18,
    ];
    const CHUNK_SIZE: usize = 4096;

    let image_data = fs::read(&image).expect("read image file");
    if image_data.len() > 0xFFFF {
        println!("error: image too large.");
        return;
    }

    let mut serial = serialport::new(port, BAUDRATE)
        .timeout(std::time::Duration::from_secs(1))
        .open()
        .expect("open serial port");

    serial.write(USB_INIT).expect("send usb_init");
    sleep(Duration::from_millis(50));
    serial.write(&[0x55; 300]).expect("send sync");
    sleep(Duration::from_millis(300));
    serial.write(HANDSHAKE).expect("send handshake");
    sleep(Duration::from_millis(100));
    serial
        .clear(serialport::ClearBuffer::Input)
        .expect("clear input buffer");

    let boot_info_raw = send_command(&mut serial, 0x10, &[]).expect("get boot info");
    if boot_info_raw.len() != 24 {
        println!(
            "error: read boot info failed. check if the port is correct and the device is supported."
        );
        return;
    }
    let chip_id: String = boot_info_raw[12..18]
        .iter()
        .rev()
        .map(|b| format!("{:02X}", b))
        .collect();
    let flash_info_from_boot = u32::from_le_bytes([
        boot_info_raw[8],
        boot_info_raw[9],
        boot_info_raw[10],
        boot_info_raw[11],
    ]);
    let flash_pin = (flash_info_from_boot >> 14) & 0x1f;
    println!(
        "chip id: {}, flash info: {:08X}, flash pin: {:02X}",
        chip_id, flash_info_from_boot, flash_pin
    );

    send_command(
        &mut serial,
        0x3b,
        (0x00014100 | flash_pin).to_le_bytes().as_ref(),
    )
    .expect("set flash pin");

    let flash_id_raw = send_command(&mut serial, 0x36, &[]).expect("read flash id");
    if flash_id_raw.len() != 4 {
        println!("error: read flash id failed.");
        return;
    }
    let flash_id: String = flash_id_raw[0..3]
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    println!("flash id: {}", flash_id);

    let flash_conf = match flash_id.as_str() {
        "EF4018" => FLASH_CONFIG_EF4018,
        _ => {
            println!("error: flash id not supported.");
            return;
        }
    };
    send_command(&mut serial, 0x3b, flash_conf).expect("set flash config");

    let begin_addr = 0x0_u32.to_le_bytes();
    let end_addr = (0x0_u32 + image_data.len() as u32).to_le_bytes();
    send_command(
        &mut serial,
        0x30,
        &[&begin_addr[..], &end_addr[..]].concat(),
    )
    .expect("erase flash");

    let mut offset = 0;
    while offset < image_data.len() {
        let len = min(CHUNK_SIZE, image_data.len() - offset);
        let chunk = &image_data[offset..offset + len];
        send_command(
            &mut serial,
            0x31,
            &[&(0x0_u32 + offset as u32).to_le_bytes(), chunk].concat(),
        )
        .expect("write image");
        offset += len;
        println!("flashing: {}/{}", offset, image_data.len());
    }

    println!("flashing done.");
}

fn send_command(
    serial: &mut Box<dyn SerialPort>,
    command: u8,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    let len = u16::try_from(data.len())
        .expect("data too long")
        .to_le_bytes();
    let mut checksum: u8 = len[0].wrapping_add(len[1]);
    for byte in data {
        checksum = checksum.wrapping_add(*byte);
    }

    let mut packet = Vec::new();
    packet.push(command);
    packet.push(checksum);
    packet.extend_from_slice(&len);
    packet.extend_from_slice(data);

    serial.write(&packet).expect("send packet");
    sleep(Duration::from_millis(200));
    let mut buf = [0u8; 4];
    serial.read(&mut buf).expect("read response");
    if !buf.starts_with(b"OK") {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "response not OK",
        )));
    }
    let response_len = u16::from_le_bytes([buf[2], buf[3]]) as usize;
    let mut response = vec![0u8; response_len];
    serial
        .read_exact(&mut response)
        .expect("read response data");
    Ok(response)
}

const FLASH_CONFIG_EF4018: &[u8] = &[
    0x04, 0x41, 0x01, 0x00, 0x04, 0x01, 0x00, 0x00, 0x66, 0x99, 0xFF, 0x03, 0x9F, 0x00, 0xB7, 0xE9,
    0x04, 0xEF, 0x00, 0x01, 0xC7, 0x20, 0x52, 0xD8, 0x06, 0x02, 0x32, 0x00, 0x0B, 0x01, 0x0B, 0x01,
    0x3B, 0x01, 0xBB, 0x00, 0x6B, 0x01, 0xEB, 0x02, 0xEB, 0x02, 0x02, 0x50, 0x00, 0x01, 0x00, 0x01,
    0x01, 0x00, 0x02, 0x01, 0x01, 0x01, 0xAB, 0x01, 0x05, 0x35, 0x00, 0x00, 0x01, 0x31, 0x00, 0x00,
    0x38, 0xFF, 0xA0, 0xFF, 0x77, 0x03, 0x02, 0x40, 0x77, 0x03, 0x02, 0xF0, 0x2C, 0x01, 0xB0, 0x04,
    0xB0, 0x04, 0x05, 0x00, 0xE8, 0x80, 0x03, 0x00,
];
