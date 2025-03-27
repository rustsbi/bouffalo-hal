use blri::{
    BootInfo, EraseFlash, Error, GetBootInfo, IspCommand, IspError, WriteFlash, elf_to_bin,
};
use clap::{Args, Parser, Subcommand};
use inquire::Select;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
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
    input: PathBuf,
    /// The path to save the patched image file. If not provided, the input file will be overwritten.
    output: Option<PathBuf>,
}

#[derive(Args)]
struct Flash {
    /// The path to the image file that needs to be flashed.
    image: PathBuf,
    /// The serial port to use for flashing. If not provided, a list of available ports will be shown.
    #[clap(short, long)]
    port: Option<String>,
}

#[derive(Args)]
struct Elf2Bin {
    /// The path to the input ELF file.
    input: PathBuf,
    /// The path to save the output binary file. If not provided, uses the input filename with .bin extension.
    #[clap(short, long)]
    output: Option<PathBuf>,
    /// Whether to patch the output binary automatically.
    #[clap(short, long)]
    patch: bool,
}

/* TODO: struct Run { input_file: PathBuf, port: Option<String> } */

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Patch(patch) => {
            let input_path = &patch.input;
            let output_path = patch.output.as_ref().unwrap_or(&input_path);
            patch_image(input_path, output_path);
        }
        Commands::Flash(flash) => {
            let port = use_or_select_flash_port(&flash.port);
            flash_image(&flash.image, &port);
        }
        Commands::Elf2bin(elf2bin) => {
            let input_path = elf2bin.input;
            // if output_file is not provided, use input filename with .bin extension
            let output_path = elf2bin
                .output
                .unwrap_or_else(|| input_path.with_extension("bin"));
            elf_to_bin(&input_path, &output_path).expect("convert ELF to BIN");
            if elf2bin.patch {
                // TODO: add a inner `patch_image` for bytes to patch the output
                // TODO: binary before saving into file system.
                patch_image(&output_path, &output_path);
            }
        }
    }
    // TODO: ^ subcommand 'blri run'
    /* Commands::Run(run) => {
        let output_file = input_file.with_extension
        let port = use_or_select_flash_port
        elf_to_bin(input, output)
        patch_image(output, output)
        flash_image(output, port)
    } */
}

fn patch_image(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) {
    let mut f_in = File::open(&input_path).expect("open input file");

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

    // Copy the input file to output file, if those files are not the same.
    // If files are the same, the following operations will reuse the input file
    // as output file, avoiding creating new files.
    let same_file = same_file::is_same_file(&output_path, &input_path).unwrap_or_else(|_| false);
    if !same_file {
        fs::copy(&input_path, &output_path).expect("copy input to output");
    }

    // release input file
    drop(f_in);

    // open output file as writeable
    let mut f_out = File::options()
        .write(true)
        .create(true)
        .open(&output_path)
        .expect("open output file");

    blri::process(&mut f_out, &ops).expect("process file");
    println!("patched image saved to {}", output_path.as_ref().display());
}

fn use_or_select_flash_port(port_parameter: &Option<String>) -> String {
    match port_parameter {
        Some(port) => port.clone(),
        None => {
            let ports = serialport::available_ports().expect("list serial ports");
            let mut port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
            port_names.sort();
            Select::new("Select a serial port", port_names)
                .prompt()
                .expect("select serial port")
        }
    }
}

fn flash_image(image: impl AsRef<Path>, port: &str) {
    const BAUDRATE: u32 = 2000000;

    let image_data = fs::read(image).expect("read image file");
    if image_data.len() > 0xFFFF {
        println!("error: image too large.");
        return;
    }

    let serial = serialport::new(port, BAUDRATE)
        .timeout(std::time::Duration::from_secs(1))
        .open()
        .expect("open serial port");

    let mut isp = UartIsp::new(serial);

    let boot_info = isp.get_boot_info().expect("get boot info");
    print_boot_info(&boot_info);

    isp.set_flash_pin(boot_info.flash_pin())
        .expect("set flash pin");

    let flash_id = isp.read_flash_id().expect("read flash id");
    println!("flash id: {:x?}", flash_id);

    let flash_config = get_flash_config_for_flash_id(flash_id).expect("retrieve config for flash");

    isp.set_flash_config(flash_config)
        .expect("set flash config");

    isp.erase_flash(0, image_data.len() as u32)
        .expect("erase flash");

    isp.write_flash(&image_data).expect("write image");

    println!("flashing done.");
}

fn print_boot_info(boot_info: &BootInfo) {
    let chip_id = &boot_info.chip_id;
    let flash_info_from_boot = boot_info.flash_info_from_boot;
    let flash_pin = boot_info.flash_pin();
    println!(
        "chip id: {:x?}, flash info: {:08X}, flash pin: {:02X}",
        chip_id, flash_info_from_boot, flash_pin
    );
}

fn get_flash_config_for_flash_id(flash_id: [u8; 3]) -> Option<&'static [u8]> {
    const FLASH_CONFIG_W25Q128_EF4018: &[u8] = &[
        0x04, 0x41, 0x01, 0x00, 0x04, 0x01, 0x00, 0x00, 0x66, 0x99, 0xFF, 0x03, 0x9F, 0x00, 0xB7,
        0xE9, 0x04, 0xEF, 0x00, 0x01, 0xC7, 0x20, 0x52, 0xD8, 0x06, 0x02, 0x32, 0x00, 0x0B, 0x01,
        0x0B, 0x01, 0x3B, 0x01, 0xBB, 0x00, 0x6B, 0x01, 0xEB, 0x02, 0xEB, 0x02, 0x02, 0x50, 0x00,
        0x01, 0x00, 0x01, 0x01, 0x00, 0x02, 0x01, 0x01, 0x01, 0xAB, 0x01, 0x05, 0x35, 0x00, 0x00,
        0x01, 0x31, 0x00, 0x00, 0x38, 0xFF, 0xA0, 0xFF, 0x77, 0x03, 0x02, 0x40, 0x77, 0x03, 0x02,
        0xF0, 0x2C, 0x01, 0xB0, 0x04, 0xB0, 0x04, 0x05, 0x00, 0xE8, 0x80, 0x03, 0x00,
    ];
    match flash_id {
        [0xef, 0x40, 0x18] => Some(FLASH_CONFIG_W25Q128_EF4018),
        _ => None,
    }
}

struct UartIsp {
    serial: Box<dyn serialport::SerialPort>,
}

impl UartIsp {
    pub fn new(mut serial: Box<dyn serialport::SerialPort>) -> Self {
        const USB_INIT: &[u8] = b"BOUFFALOLAB5555RESET\0\x01";
        const HANDSHAKE: &[u8] = &[
            0x50, 0x00, 0x08, 0x00, 0x38, 0xF0, 0x00, 0x20, 0x00, 0x00, 0x00, 0x18,
        ];

        serial.write(USB_INIT).expect("send usb_init");
        sleep(Duration::from_millis(50));
        serial.write(&[0x55; 300]).expect("send sync");
        sleep(Duration::from_millis(300));
        serial.write(HANDSHAKE).expect("send handshake");
        sleep(Duration::from_millis(100));
        serial
            .clear(serialport::ClearBuffer::Input)
            .expect("clear input buffer");
        Self { serial }
    }

    pub fn get_boot_info(&mut self) -> Result<BootInfo, UartIspError> {
        send_command(&mut self.serial, GetBootInfo)
    }

    pub fn set_flash_pin(&mut self, flash_pin: u32) -> Result<(), UartIspError> {
        send_command_raw(
            &mut self.serial,
            0x3b,
            (0x00014100 | flash_pin).to_le_bytes().as_ref(),
            false,
        )?;
        Ok(())
    }

    pub fn read_flash_id(&mut self) -> Result<[u8; 3], UartIspError> {
        let ans = send_command_raw(&mut self.serial, 0x36, &[], true)?;
        if ans.len() != 4 {
            panic!("incorrect length for read_flash_id")
        }
        Ok([ans[0], ans[1], ans[2]])
    }

    pub fn set_flash_config(&mut self, flash_config: &[u8]) -> Result<(), UartIspError> {
        send_command_raw(&mut self.serial, 0x3b, flash_config, false)?;
        Ok(())
    }

    pub fn erase_flash(&mut self, start: u32, end: u32) -> Result<(), UartIspError> {
        send_command(&mut self.serial, EraseFlash::new(start, end))?;
        Ok(())
    }

    pub fn write_flash(&mut self, image: &[u8]) -> Result<usize, UartIspError> {
        const CHUNK_SIZE: usize = 4096;
        for (chunk_idx, chunk) in image.chunks(CHUNK_SIZE).enumerate() {
            let offset = (chunk_idx * CHUNK_SIZE) as u32;
            send_command(&mut self.serial, WriteFlash::new(offset, chunk))?;
            println!("flashing: {}/{}", offset, image.len());
        }
        Ok(image.len())
    }
}

#[derive(thiserror::Error, Debug)]
enum UartIspError {
    #[error("UART response error")]
    UartResponse(#[from] UartResponseError),
    #[error("UART I/O error")]
    UartIo(#[from] std::io::Error),
    #[error("Isp protocol error")]
    IspError(#[from] IspError),
}

fn send_command<T: IspCommand>(
    serial: impl Read + Write,
    command: T,
) -> Result<T::Response, UartIspError> {
    let mut data = vec![0u8; command.data_size()];
    command.write_packet_data(&mut data);
    let bytes = send_command_raw(serial, T::COMMAND, &data, T::RESPONSE_PAYLOAD)?;
    let ans = T::parse_response(&bytes)?;
    Ok(ans)
}

fn send_command_raw(
    mut serial: impl Read + Write,
    command: u8,
    data: &[u8],
    response_payload: bool,
) -> Result<Vec<u8>, UartIspError> {
    if data.len() > u16::MAX as usize {
        panic!("data too long");
    }

    let packet_header = packet_header(command, data);
    serial.write(&packet_header).expect("send packet header");
    serial.write(&data).expect("send packet content");

    // sleep(Duration::from_millis(200));
    let response_len = query_response(&mut serial, response_payload)?;
    let mut response = vec![0u8; response_len as usize];
    serial
        .read_exact(&mut response)
        .expect("read response data");
    Ok(response)
}

// TODO: unit test for packet_header function
fn packet_header(command: u8, data: &[u8]) -> [u8; 4] {
    assert!(data.len() <= u16::MAX as usize);
    let len_bytes = (data.len() as u16).to_le_bytes();
    let checksum = len_bytes
        .iter()
        .chain(data)
        .fold(0u8, |a, b| b.wrapping_add(a));

    [command, checksum, len_bytes[0], len_bytes[1]]
}

// Ref: https://github.com/pine64/blisp/blob/e45941c45e2418b2bb7e3dab49468a8f4d132439/lib/blisp.c#L144
#[derive(thiserror::Error, Debug)]
enum UartResponseError {
    #[error("Operation pending")]
    Pending,
    #[error("Operation failed")]
    Failed,
    #[error("Unknown operation")]
    Unknown([u8; 2]),
}

fn query_response(mut serial: impl Read, response_payload: bool) -> Result<u16, UartIspError> {
    let mut state = [0u8; 2];
    serial.read_exact(&mut state[..2])?;
    match &state {
        b"OK" => {}
        b"PD" => return Err(UartResponseError::Pending)?,
        b"FL" => return Err(UartResponseError::Failed)?,
        others => return Err(UartResponseError::Unknown(*others))?,
    }
    if response_payload {
        let mut len_buf = [0u8; 2];
        serial.read_exact(&mut len_buf)?;
        return Ok(u16::from_le_bytes(len_buf));
    }
    Ok(0)
}
