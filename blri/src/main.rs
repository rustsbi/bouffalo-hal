use blri::{
    BootInfo, DeviceReset, EraseFlash, Error, GetBootInfo, ImageToFuse, IspCommand, IspError,
    WriteFlash, elf_to_bin, settings::BlriConfig,
};
use clap::{Args, Parser, Subcommand};
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use inquire::Select;
use std::{
    fs::{self, File},
    io::{Read, Write, stdin, stdout},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, sleep},
    time::Duration,
};

/// Device startup delay in milliseconds - time to wait for device to boot and stabilize
// Timing constants
const DEVICE_STARTUP_DELAY_MS: u64 = 1500; // Delay between device initialization and starting console
const ISP_HANDSHAKE_DELAY_MS: u64 = 50; // ISP handshake timing
const ISP_SETUP_DELAY_MS: u64 = 300; // ISP setup delay
const GENERAL_OPERATION_DELAY_MS: u64 = 100; // General operation delays
const SERIAL_TIMEOUT_MS: u64 = 100; // Serial communication timeout
const DEVICE_READY_DELAY_MS: u64 = 500; // Device ready wait time
const RETRY_DELAY_MS: u64 = 200; // Retry operation delay
const POLL_INTERVAL_MS: u64 = 10; // Polling interval for status checks

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
    /// Convert ELF to binary file, patch, flash image and open serial.
    Run(Run),
    /// Use saved configuration to run the default project.
    Default(DefaultRun),
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
    #[arg(short, long)]
    port: Option<String>,
    #[arg(long, default_value_t = false)]
    reset: bool,
}

#[derive(Args)]
struct Elf2Bin {
    /// The path to the input ELF file.
    input: PathBuf,
    /// The path to save the output binary file. If not provided, uses the input filename with .bin extension.
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Whether to patch the output binary automatically.
    #[arg(short, long)]
    patch: bool,
}

#[derive(Args)]
struct Run {
    /// The path to the input ELF file. If not provided, will use the saved configuration.
    input_file: Option<PathBuf>,
    /// The serial port to use for flashing. If not provided, a list of available ports will be shown.
    #[arg(short, long)]
    port: Option<String>,
    #[arg(long, default_value_t = false)]
    reset: bool,
    /// Open serial console after flashing is complete
    #[arg(long, default_value_t = false)]
    console: bool,
    /// Enable verbose debug output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Args)]
struct DefaultRun {
    /// Override reset setting
    #[arg(long)]
    reset: Option<bool>,
    /// Override console setting
    #[arg(long)]
    console: Option<bool>,
    /// Override verbose setting
    #[arg(short, long)]
    verbose: Option<bool>,
}

#[derive(Args)]
/// Represents the command to fuse multiple images into a single image.
/// Currently only supports fuse a M0 image and a D0 image into a fused image.
struct Fuse {
    #[command(flatten)]
    m0_image: Option<ImageToFuse>,
    #[command(flatten)]
    d0_image: Option<ImageToFuse>,
    // Note: `lp_image` is currently not supported!!!
    #[command(flatten)]
    lp_image: Option<ImageToFuse>,
    fused_image: PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Load existing configuration
    let mut config = BlriConfig::load();

    match args.command {
        Commands::Patch(patch) => {
            let input_path = &patch.input;
            let output_path = patch.output.as_ref().unwrap_or(&input_path);
            patch_image(input_path, output_path);
        }
        Commands::Flash(flash) => {
            let (port, baudrate) = use_or_select_flash_port_and_baudrate(&flash.port, None);
            if let Err(e) = flash_image(&flash.image, &port, flash.reset, baudrate) {
                println!("error: {}", e);
            }
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
        Commands::Run(run) => {
            handle_run_command(run, &mut config);
        }
        Commands::Default(default_run) => {
            handle_default_command(default_run, &mut config);
        }
    }
}

fn patch_image(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) {
    let mut f_in = File::open(&input_path).expect("open input file");

    let ops = match blri::check(&mut f_in) {
        Ok(ops) => ops,
        Err(e) => {
            print_patch_error(e);
            return;
        }
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

fn print_patch_error(e: Error) {
    match e {
        Error::MagicNumber { wrong_magic } => {
            println!("error: incorrect magic number 0x{wrong_magic:08x}!");
        }
        Error::HeadLength { wrong_length } => {
            println!(
                "File is too short to include an image header, it only includes {wrong_length} bytes"
            );
        }
        Error::FlashConfigMagic { wrong_magic } => {
            println!("error: incorrect flash config magic 0x{wrong_magic:08x}!");
        }
        Error::ClockConfigMagic { wrong_magic } => {
            println!("error: incorrect clock config magic 0x{wrong_magic:08x}!");
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
        }
        Error::Sha256Checksum { wrong_checksum } => {
            let mut wrong_checksum_hex = String::new();
            for i in wrong_checksum {
                wrong_checksum_hex.push_str(&format!("{:02x}", i));
            }
            println!("error: wrong sha256 verification: {}.", wrong_checksum_hex);
        }
        Error::Io(source) => {
            println!("error: io error! {:?}", source);
        }
    }
}

fn use_or_select_flash_port_and_baudrate(
    port_parameter: &Option<String>,
    baudrate_parameter: Option<u32>,
) -> (String, u32) {
    let port = match port_parameter {
        Some(port) => port.clone(),
        None => {
            let ports = serialport::available_ports().expect("list serial ports");
            let mut port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
            port_names.sort();
            Select::new("Select a serial port", port_names)
                .prompt()
                .expect("select serial port")
        }
    };

    let baudrate = match baudrate_parameter {
        Some(rate) => rate,
        None => select_baudrate(),
    };

    (port, baudrate)
}

fn select_baudrate() -> u32 {
    // Common baudrates from high to low, with 2000000 as default
    let baudrate_options = vec![
        "2000000 (default)".to_string(),
        "1500000".to_string(),
        "1000000".to_string(),
        "921600".to_string(),
        "460800".to_string(),
        "230400".to_string(),
        "115200".to_string(),
        "57600".to_string(),
        "38400".to_string(),
        "19200".to_string(),
        "9600".to_string(),
        "Custom".to_string(),
    ];

    let selection = Select::new("Select baudrate", baudrate_options)
        .with_starting_cursor(0) // Default to 2000000
        .prompt()
        .expect("select baudrate");

    match selection.as_str() {
        "2000000 (default)" => 2000000,
        "1500000" => 1500000,
        "1000000" => 1000000,
        "921600" => 921600,
        "460800" => 460800,
        "230400" => 230400,
        "115200" => 115200,
        "57600" => 57600,
        "38400" => 38400,
        "19200" => 19200,
        "9600" => 9600,
        "Custom" => {
            use inquire::Text;
            let input = Text::new("Enter custom baudrate:")
                .prompt()
                .expect("get custom baudrate input");

            match input.trim().parse::<u32>() {
                Ok(rate) if rate > 0 => rate,
                Ok(_) => {
                    eprintln!("Error: Baudrate must be a positive integer");
                    std::process::exit(1);
                }
                Err(_) => {
                    eprintln!(
                        "Error: Invalid baudrate '{}'. Must be a positive integer",
                        input.trim()
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn flash_image(
    image: impl AsRef<Path>,
    port: &str,
    device_reset: bool,
    baudrate: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_data = fs::read(image)?;
    if image_data.len() > u16::MAX as usize {
        return Err("Image too large".into());
    }

    println!("Connecting to device on {} at {} bps...", port, baudrate);
    println!("Please ensure your device is in download mode:");
    println!("1. Hold BOOT button and press RESET button");
    println!("2. Release RESET button, then release BOOT button");
    println!("3. The device should now be in ISP mode");

    let serial = serialport::new(port, baudrate)
        .timeout(std::time::Duration::from_secs(1))
        .open()
        .map_err(|e| format!("Failed to open serial port {}: {}", port, e))?;

    let mut isp = UartIsp::new(serial);

    let boot_info = isp.get_boot_info().map_err(|e| {
        format!("Failed to get boot info: {:?}\nPlease check:\n1. Device is connected and powered on\n2. Device is in ISP/download mode (BOOT button sequence)\n3. Correct serial port is selected\n4. No other programs are using the serial port", e)
    })?;

    print_boot_info(&boot_info);

    isp.set_flash_pin(boot_info.flash_pin())
        .map_err(|e| format!("Failed to set flash pin: {:?}", e))?;

    let flash_id = isp
        .read_flash_id()
        .map_err(|e| format!("Failed to read flash id: {:?}", e))?;
    println!("flash id: {:x?}", flash_id);

    let flash_config = get_flash_config_for_flash_id(flash_id).ok_or_else(|| {
        format!(
            "Unsupported flash id {:x?}. Supported: W25Q128 (EF4018)",
            flash_id
        )
    })?;

    isp.set_flash_config(flash_config)
        .map_err(|e| format!("Failed to set flash config: {:?}", e))?;

    println!("Erasing flash...");
    isp.erase_flash(0, image_data.len() as u32)
        .map_err(|e| format!("Failed to erase flash: {:?}", e))?;

    println!("Writing image...");
    isp.write_flash(&image_data)
        .map_err(|e| format!("Failed to write image: {:?}", e))?;

    println!("flashing done.");

    if device_reset {
        if let Err(e) = isp.device_reset() {
            println!("warning: failed to reset device: {:?}", e);
        } else {
            println!("resetting device...");
        }
    }

    Ok(())
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
        sleep(Duration::from_millis(ISP_HANDSHAKE_DELAY_MS));
        serial.write(&[0x55; 300]).expect("send sync");
        sleep(Duration::from_millis(ISP_SETUP_DELAY_MS));
        serial.write(HANDSHAKE).expect("send handshake");
        sleep(Duration::from_millis(GENERAL_OPERATION_DELAY_MS));
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

    pub fn device_reset(&mut self) -> Result<(), UartIspError> {
        send_command(&mut self.serial, DeviceReset)
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

fn open_serial_console(port: &str, is_console_mode: bool, baudrate: u32) {
    // Wait for device to start and collect initial output
    println!("Waiting for device to start...");
    thread::sleep(Duration::from_millis(DEVICE_STARTUP_DELAY_MS));

    let mut serial = match serialport::new(port, baudrate)
        .timeout(Duration::from_millis(SERIAL_TIMEOUT_MS))
        .open()
    {
        Ok(port) => port,
        Err(e) => {
            println!("Failed to open serial port for console: {}", e);
            return;
        }
    };

    thread::sleep(Duration::from_millis(DEVICE_READY_DELAY_MS));

    // Collect all buffered data before showing banner
    let mut all_buffered_data = String::new();
    let mut buffer = [0u8; 2048];

    for i in 0..10 {
        match serial.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let data = String::from_utf8_lossy(&buffer[..bytes_read]);
                all_buffered_data.push_str(&data);
                if i < 3 {
                    thread::sleep(Duration::from_millis(GENERAL_OPERATION_DELAY_MS));
                } else {
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                }
            }
            _ => {
                thread::sleep(Duration::from_millis(GENERAL_OPERATION_DELAY_MS));
                if i > 5 && all_buffered_data.is_empty() {
                    break;
                }
            }
        }
    }

    // Show formatted console start message
    show_console_banner(port, baudrate, is_console_mode);

    // Display collected buffered data
    if !all_buffered_data.trim().is_empty() {
        let mut remaining_data = all_buffered_data.as_str();

        while !remaining_data.is_empty() {
            if let Some(newline_pos) = remaining_data.find('\n') {
                let line = &remaining_data[..newline_pos];
                if !line.trim().is_empty() {
                    let formatted_line = format_serial_output(line.trim_end_matches('\r'));
                    println!("{}", formatted_line);
                }
                remaining_data = &remaining_data[newline_pos + 1..];
            } else {
                if !remaining_data.trim().is_empty() {
                    let formatted_partial =
                        format_serial_output(remaining_data.trim_end_matches('\r'));
                    print!("{}", formatted_partial);
                    let _ = stdout().flush();
                }
                break;
            }
        }
    }

    // Use monitor mode for non-interactive display
    if !is_console_mode {
        open_serial_monitor_only(port, serial);
        return;
    }

    // Enable console mode with raw input
    if let Err(e) = enable_raw_mode() {
        println!("Warning: failed to enable raw mode: {}", e);
        println!("Falling back to line-buffered input...");
        open_serial_console_fallback(port, &mut serial, is_console_mode);
        return;
    }

    let running = Arc::new(AtomicBool::new(true));

    let mut serial_clone = match serial.try_clone() {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to clone serial port: {}", e);
            let _ = disable_raw_mode();
            return;
        }
    };
    let running_clone = running.clone();

    // Handle serial port reading in separate thread
    let read_handle = thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        let mut line_buffer = String::new();

        while running_clone.load(Ordering::SeqCst) {
            match serial_clone.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let data = &buffer[..bytes_read];
                    let output = String::from_utf8_lossy(data);

                    for ch in output.chars() {
                        match ch {
                            '\n' => {
                                let formatted_line = format_serial_output(&line_buffer);
                                print!("{}\r\n", formatted_line);
                                line_buffer.clear();
                            }
                            '\r' => {
                                let formatted_line = format_serial_output(&line_buffer);
                                print!("{}\r", formatted_line);
                                line_buffer.clear();
                            }
                            '\t' => {
                                line_buffer.push('\t');
                            }
                            c if c.is_control() && c != '\x08' => {
                                continue;
                            }
                            c => {
                                line_buffer.push(c);
                            }
                        }
                    }

                    if !line_buffer.is_empty() && !output.ends_with('\n') && !output.ends_with('\r')
                    {
                        let formatted_partial = format_serial_output(&line_buffer);
                        print!("{}", formatted_partial);
                        line_buffer.clear();
                    }

                    let _ = stdout().flush();
                }
                Ok(_) => {
                    thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    // Timeout is expected, continue
                    continue;
                }
                Err(e) => {
                    eprintln!("Error reading from serial port: {}", e);
                    break;
                }
            }
        }
    });

    // Handle keyboard input and send to device
    while running.load(Ordering::SeqCst) {
        if event::poll(Duration::from_millis(GENERAL_OPERATION_DELAY_MS)).unwrap_or(false) {
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && key_event.code == KeyCode::Char('c')
                    {
                        running.store(false, Ordering::SeqCst);
                        break;
                    }

                    let bytes_to_send = match key_event.code {
                        KeyCode::Char(c) => vec![c as u8],
                        KeyCode::Enter => vec![b'\r'],
                        KeyCode::Backspace => vec![0x08],
                        KeyCode::Tab => vec![b'\t'],
                        KeyCode::Esc => vec![0x1b],
                        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => continue,
                        KeyCode::Home | KeyCode::End | KeyCode::PageUp | KeyCode::PageDown => {
                            continue;
                        }
                        KeyCode::Insert | KeyCode::Delete => continue,
                        KeyCode::F(_) => continue,
                        _ => continue,
                    };

                    if let Err(e) = serial.write_all(&bytes_to_send) {
                        eprintln!("Error writing to serial port: {}", e);
                        break;
                    }
                    let _ = serial.flush();
                }
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }

    running.store(false, Ordering::SeqCst);
    let _ = disable_raw_mode();
    let _ = read_handle.join();

    println!("\n{}", "Serial console closed.".bright_green());
}

fn show_console_banner(port: &str, baudrate: u32, is_console_mode: bool) {
    const SEPARATOR_WIDTH: usize = 50; // Fixed width for consistency

    let separator = "─".repeat(SEPARATOR_WIDTH);

    println!();
    println!("{}", separator.bright_cyan());

    if is_console_mode {
        println!("  🖥️  {}", "Serial Console".bright_white().bold());
    } else {
        println!("  📡 {}", "Serial Monitor".bright_white().bold());
    }

    println!("{}", separator.bright_cyan());
    println!(
        "  ⚡ {}: {}",
        "Speed".bright_yellow(),
        format!("{} bps", baudrate).bright_white()
    );
    println!("  🔌 {}: {}", "Port".bright_yellow(), port.bright_white());
    println!("  ✅ {}", "Ready".bright_green().bold());
    println!("  🚪 Press {} to exit", "Ctrl+C".bright_red().bold());
    println!("{}", separator.bright_cyan());
    println!();

    if is_console_mode {
        println!("📡 {}", "Console Output:".bright_magenta().bold());
    } else {
        println!("📡 {}", "Device Output:".bright_magenta().bold());
    }
    println!();
}

fn format_serial_output(text: &str) -> String {
    if text.trim().is_empty() {
        return text.to_string();
    }

    if text.starts_with('>') {
        text.bright_green().bold().to_string()
    } else if text.contains("error") || text.contains("Error") || text.contains("ERROR") {
        text.bright_red().to_string()
    } else if text.contains("warning") || text.contains("Warning") || text.contains("WARN") {
        text.bright_yellow().to_string()
    } else if text.contains("Command") || text.contains("help") {
        text.bright_cyan().to_string()
    } else if text.contains("Unknown command") {
        text.bright_red().to_string()
    } else if text.contains(':')
        && (text.contains("on") || text.contains("off") || text.contains("switch"))
    {
        text.bright_blue().to_string()
    } else {
        text.white().to_string()
    }
}

fn open_serial_console_fallback(
    _port: &str,
    serial: &mut Box<dyn serialport::SerialPort>,
    _is_console_mode: bool,
) {
    println!(
        "{}",
        "Using line-buffered input mode. Press Ctrl+C to exit.".bright_yellow()
    );

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\n{}", "Exiting serial console...".bright_green());
    })
    .expect("Error setting Ctrl-C handler");

    let mut serial_clone = serial.try_clone().expect("Failed to clone serial port");
    let running_clone = running.clone();

    let read_handle = thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        while running_clone.load(Ordering::SeqCst) {
            match serial_clone.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let data = &buffer[..bytes_read];
                    let output = String::from_utf8_lossy(data);

                    // Apply highlighting to each line
                    for line in output.lines() {
                        let formatted_line = format_serial_output(line);
                        println!("{}", formatted_line);
                    }

                    // Handle partial lines (without newline)
                    if !output.ends_with('\n') && !output.is_empty() {
                        let last_part = output.lines().last().unwrap_or("");
                        if !last_part.is_empty() {
                            let formatted = format_serial_output(last_part);
                            print!("{}", formatted);
                        }
                    }

                    let _ = stdout().flush();
                }
                Ok(_) => {
                    thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    eprintln!("Error reading from serial port: {}", e);
                    break;
                }
            }
        }
    });

    // Main thread handles user input
    let stdin = stdin();
    let mut input_buffer = String::new();
    while running.load(Ordering::SeqCst) {
        input_buffer.clear();
        match stdin.read_line(&mut input_buffer) {
            Ok(_) => {
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                if let Err(e) = serial.write_all(input_buffer.as_bytes()) {
                    eprintln!("Error writing to serial port: {}", e);
                    break;
                }
                let _ = serial.flush();
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    running.store(false, Ordering::SeqCst);
    let _ = read_handle.join();

    println!("\n{}", "Serial console closed.".bright_green());
}

fn open_serial_monitor_only(_port: &str, mut serial: Box<dyn serialport::SerialPort>) {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let mut buffer = [0u8; 1024];
    let mut line_buffer = String::new();

    while running_clone.load(Ordering::SeqCst) {
        match serial.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let data = &buffer[..bytes_read];
                let output = String::from_utf8_lossy(data);

                for ch in output.chars() {
                    match ch {
                        '\n' => {
                            let formatted_line = format_serial_output(&line_buffer);
                            println!("{}", formatted_line);
                            line_buffer.clear();
                        }
                        '\r' => {
                            if !line_buffer.is_empty() {
                                let formatted_line = format_serial_output(&line_buffer);
                                print!("{}\r", formatted_line);
                                let _ = stdout().flush();
                                line_buffer.clear();
                            }
                        }
                        c if c.is_control() => {
                            continue;
                        }
                        c => {
                            line_buffer.push(c);
                        }
                    }
                }

                if !line_buffer.is_empty() && !output.ends_with('\n') && !output.ends_with('\r') {
                    let formatted_partial = format_serial_output(&line_buffer);
                    print!("{}", formatted_partial);
                    let _ = stdout().flush();
                    line_buffer.clear();
                }
            }
            Ok(_) => {
                thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                println!("Error reading from serial port: {}", e);
                break;
            }
        }
    }

    println!("\n{}", "Serial monitor closed.".bright_green());
}

fn handle_default_command(default_run: DefaultRun, config: &mut BlriConfig) {
    // Check if we have saved configuration with binary path
    if let Some(binary_path) = config.get_binary_path() {
        if binary_path.exists() {
            println!("📋 Using saved configuration:");
            if let Some(target) = &config.target {
                println!("  Target: {}", target);
            }
            println!("  Release: {}", if config.release { "Yes" } else { "No" });
            if let Some(package) = &config.package {
                println!("  Package: {}", package);
            }
            println!("  Verbose: {}", if config.verbose { "Yes" } else { "No" });
            println!("  Binary: {}", binary_path.display());
            println!();

            // Use unified run logic
            run_with_unified_config(
                config,
                &binary_path,
                config.port.clone(),
                Some(config.baudrate),
                config.target.clone(),
                config.release,
                config.package.clone(),
                default_run.reset.unwrap_or(config.reset),
                default_run.console.unwrap_or(config.console),
                default_run.verbose.unwrap_or(config.verbose),
            );
        } else {
            println!(
                "{}",
                "❌ Saved configuration binary not found.".bright_red()
            );
            println!("Binary path: {}", binary_path.display());
            println!("Available options:");
            if let (Some(target), Some(package)) = (&config.target, &config.package) {
                println!(
                    "1. Build the project first: cargo build --target {} {} -p {}",
                    target,
                    if config.release { "--release" } else { "" },
                    package
                );
            }
        }
    } else {
        println!("{}", "❌ No saved configuration found.".bright_red());
        println!("Please run with a target first to create a configuration:");
        println!("  cargo run run <path-to-binary>");
    }
}

/// Unified run function with two-step confirmation
fn run_with_unified_config(
    config: &mut BlriConfig,
    input_file: &PathBuf,
    port: Option<String>,
    baudrate: Option<u32>,
    target: Option<String>,
    release: bool,
    package: Option<String>,
    reset: bool,
    console: bool,
    verbose: bool,
) {
    // Use provided port or select one
    let (final_port, final_baudrate) = if let Some(port) = port {
        (port, baudrate.unwrap_or(2000000))
    } else {
        use_or_select_flash_port_and_baudrate(&None, baudrate)
    };

    // Handle configuration conflicts and get decisions
    let (use_current, should_save_after) = match config.handle_configuration_conflict(
        &final_port,
        final_baudrate,
        target.clone(),
        release,
        package.clone(),
        reset,
        console,
        verbose,
    ) {
        Ok(result) => result,
        Err(e) => {
            println!("Error handling configuration: {}", e);
            return;
        }
    };

    // Determine which configuration to use for running
    let (run_port, run_baudrate, run_reset, run_console) = if use_current {
        // Use current (command line) configuration
        (final_port.clone(), final_baudrate, reset, console)
    } else {
        // Use saved configuration
        (
            config.port.as_ref().unwrap_or(&final_port).clone(),
            config.baudrate,
            config.reset,
            config.console,
        )
    };

    println!("🚀 Running with configuration...");

    // Convert ELF to BIN and patch
    let bin_file = input_file.with_extension("bin");
    elf_to_bin(&input_file, &bin_file).expect("convert ELF to BIN");
    patch_image(&bin_file, &bin_file);

    // Flash the image
    match flash_image(&bin_file, &run_port, run_reset, run_baudrate) {
        Ok(_) => {
            // Open serial interface based on console setting
            if run_console {
                println!("Opening serial console on {}...", run_port);
                open_serial_console(&run_port, true, run_baudrate);
            } else {
                println!("Opening serial monitor on {}...", run_port);
                open_serial_console(&run_port, false, run_baudrate);
            }

            // Second confirmation: save configuration after successful run
            if should_save_after {
                if let Err(e) = config.save_after_run(
                    &final_port,
                    final_baudrate,
                    target,
                    release,
                    package,
                    reset,
                    console,
                    verbose,
                ) {
                    println!("Error saving configuration: {}", e);
                }
            }
        }
        Err(e) => {
            println!("error: {}", e);
            println!("Flashing failed. Serial interface will not be opened.");
        }
    }
}

fn handle_run_command(run: Run, config: &mut BlriConfig) {
    // Check if input file is provided
    let input_file = match run.input_file {
        Some(file) => file,
        None => {
            // No input file provided, try to use saved configuration
            if let Some(binary_path) = config.get_binary_path() {
                if binary_path.exists() {
                    println!("📋 Using saved configuration:");
                    if let Some(target) = &config.target {
                        println!("  Target: {}", target);
                    }
                    println!("  Release: {}", if config.release { "Yes" } else { "No" });
                    if let Some(package) = &config.package {
                        println!("  Package: {}", package);
                    }
                    println!("  Verbose: {}", if config.verbose { "Yes" } else { "No" });
                    println!("  Binary: {}", binary_path.display());
                    println!();
                    binary_path
                } else {
                    println!(
                        "{}",
                        "❌ No input file provided and saved configuration binary not found."
                            .bright_red()
                    );
                    println!("Binary path: {}", binary_path.display());
                    println!("Available options:");
                    println!("1. Provide input file: cargo blri run <path-to-binary>");
                    if let (Some(target), Some(package)) = (&config.target, &config.package) {
                        println!(
                            "2. Build the project first: cargo build --target {} {} -p {}",
                            target,
                            if config.release { "--release" } else { "" },
                            package
                        );
                    }
                    return;
                }
            } else {
                println!(
                    "{}",
                    "❌ No input file provided and no saved configuration found.".bright_red()
                );
                println!("Please provide an input file or run with a target first.");
                return;
            }
        }
    };

    // Extract build parameters from environment and binary path
    let args: Vec<String> = std::env::args().collect();

    // Method 1: Try environment variables set by cargo
    let mut target = None;
    let mut package = None;
    let mut release = false;

    // Check all environment variables for debugging
    let env_vars: Vec<(String, String)> = std::env::vars().collect();

    // Extract target from binary path
    let binary_path = input_file.to_string_lossy();

    // Parse target from path like: target/riscv64imac-unknown-none-elf/release/uart-demo
    let path_parts: Vec<&str> = binary_path.split('/').collect();

    // Find target architecture
    for (i, part) in path_parts.iter().enumerate() {
        if *part == "target" && i + 1 < path_parts.len() {
            let potential_target = path_parts[i + 1];
            if potential_target.contains("-unknown-none-elf")
                || potential_target == "debug"
                || potential_target == "release"
            {
                if potential_target != "debug" && potential_target != "release" {
                    target = Some(potential_target.to_string());
                }
            }
        }
    }

    // Find release/debug mode
    if binary_path.contains("/release/") {
        release = true;
    }

    // Debug output only when verbose flag is enabled
    if run.verbose {
        println!("🔧 Program args: {:?}", args);
        println!("🔧 Binary path: {}", binary_path);

        // Print environment variables for debugging
        for (key, value) in &env_vars {
            if key.starts_with("CARGO_") {
                println!("  {}: {}", key, value);
            }
        }
    }

    // Extract package name from binary file name
    if let Some(file_name) = input_file.file_name() {
        package = Some(file_name.to_string_lossy().to_string());
    }

    // Try environment variables as backup
    if target.is_none() {
        target = std::env::var("CARGO_CFG_TARGET_ARCH")
            .ok()
            .or_else(|| std::env::var("TARGET").ok());
    }

    if package.is_none() {
        package = std::env::var("CARGO_PKG_NAME").ok();
    }

    if !release {
        release = std::env::var("PROFILE")
            .map(|p| p == "release")
            .unwrap_or(false);
    }

    // Debug output for detected build parameters (only when verbose flag is enabled)
    if run.verbose {
        println!("🔧 Detected build parameters:");
        println!("  Target: {:?}", target);
        println!("  Release: {}", release);
        println!("  Package: {:?}", package);
        println!();
    }

    // Use unified run logic
    run_with_unified_config(
        config,
        &input_file,
        run.port,
        None, // Let the function determine baudrate
        target,
        release,
        package,
        run.reset,
        run.console,
        run.verbose,
    );
}
