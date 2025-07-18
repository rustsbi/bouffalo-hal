use super::{Config, Device, Error, config::*, ser::set_bootargs};
use bouffalo_hal::sdio::sdcard::Sdcard;
use embedded_hal::digital::OutputPin;
use embedded_io::{Read, Write};
use embedded_sdmmc::{
    BlockDevice, Mode, RawDirectory, RawFile, TimeSource, Timestamp, VolumeManager,
};

/// Time source implementation for SD card filesystem.
pub struct MyTimeSource {}

impl TimeSource for MyTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        // Returns a fixed timestamp for simplicity.
        Timestamp::from_calendar(2023, 1, 1, 0, 0, 0).unwrap()
    }
}

/// Loads necessary files from SD card into memory.
pub fn load_from_sdcard<
    W: Write,
    R: Read,
    L: OutputPin,
    SDH: core::ops::Deref<Target = bouffalo_hal::sdio::RegisterBlock>,
    PADS,
>(
    d: &mut Device<W, R, L, SDH, PADS>,
) -> Result<usize, ()> {
    // SD card initialization.
    writeln!(d.tx, "initializing sdcard...").ok();
    d.sdh.init(&mut d.tx, true);
    let sdcard = Sdcard::new(&mut d.sdh);
    // Initialize filesystem and open root directory.
    let mut volume_mgr = VolumeManager::new(sdcard, MyTimeSource {});
    let volume0 = volume_mgr
        .open_raw_volume(embedded_sdmmc::VolumeIdx(0))
        .map_err(|_| ())?;
    let root_dir = volume_mgr.open_root_dir(volume0).map_err(|_| ())?;

    // Read configuration from `config.toml`.
    let bl808_cfg = "CONFIG~1.TOM";
    let buffer = &mut [0u8; 512];
    let size = if let Ok(toml) = volume_mgr.open_file_in_dir(root_dir, bl808_cfg, Mode::ReadOnly) {
        if let Ok(size) =
            load_file_into_memory(&mut volume_mgr, toml, buffer.as_mut_ptr() as usize, 512)
        {
            size
        } else {
            writeln!(d.tx, "error: cannot load config file `config.toml`.").ok();
            return Err(());
        }
    } else {
        writeln!(d.tx, "error: cannot find config file `config.toml`.").ok();
        return Err(());
    };

    // Parse configuration.
    let Ok(toml_str) = core::str::from_utf8(&buffer[..size]) else {
        writeln!(d.tx, "error: invalid config encoding.").ok();
        return Err(());
    };

    let Ok(config) = picotoml::from_str::<Config>(toml_str) else {
        writeln!(d.tx, "error: invalid toml format.").ok();
        return Err(());
    };

    // Load device tree blob.
    let Some(opaque) = config.configs.opaque else {
        writeln!(d.tx, "warning: /config.toml: cannot find opaque file path on key `configs.opaque`, using default configuration (zeroing `a1` for non-existing opaque file).").ok();
        volume_mgr.close_dir(root_dir).unwrap();
        return Ok(0x0);
    };
    let dtb_path = opaque.as_str();
    let (file_dir, file_name) = locate_file_by_path(&mut d.tx, "dtb", dtb_path);
    let Ok(dtb) = open_file_by_path(&mut volume_mgr, root_dir, file_dir, file_name) else {
        writeln!(
            d.tx,
            "error: /config.toml: file not found for dtb path {}.",
            dtb_path
        )
        .ok();
        return Err(());
    };
    writeln!(d.tx, "info: /config.toml: dtb located on {}.", dtb_path).ok();
    // Load `bl808.dtb`.
    let result = load_file_into_memory(&mut volume_mgr, dtb, OPAQUE_ADDRESS, OPAQUE_LENGTH);
    match result {
        Ok(bytes) => {
            writeln!(
                d.tx,
                "info: load {} success, size = {} bytes",
                file_name, bytes
            )
            .ok();
        }
        Err(Error::FileLength(size)) => {
            writeln!(d.tx, "error: /config.toml: file size for dtb {} is {} bytes, but maximum supported dtb size on the current platform (BL808) is 64 KiB.", dtb_path, size).ok();
            return Err(());
        }
        Err(Error::BlockDevice(e)) => {
            writeln!(
                d.tx,
                "error: cannot load file `{}` for underlying block device error: {:?}",
                file_name, e
            )
            .ok();
            return Err(());
        }
        Err(e) => {
            writeln!(
                d.tx,
                "error: an unexpected error occurred while loading the DTB: {:?}",
                e
            )
            .ok();
            return Err(());
        }
    }
    // Patch bootargs to dtb.
    if let Some(bootargs) = config.configs.bootargs {
        writeln!(d.tx, "debug: /config.toml: bootargs load start.").ok();
        let result = set_bootargs(&bootargs);
        writeln!(
            d.tx,
            "debug: /config.toml: bootargs `{}` load from dtb.",
            bootargs
        )
        .ok();
        match result {
            Ok(()) => {
                writeln!(d.tx, "info: /config.toml: bootargs set to `{}`.", bootargs).ok();
            }
            Err(Error::InvalidMagic(_magic)) => {
                writeln!(d.tx, "warning: /config.toml: bootargs is unused, as `config.opaque` does not include an opaque information file in DTB format.
        note: /config.toml: `config.bootargs` is set to `console=ttyS0,115200n8 root=/dev/mmcblk0p2 rw rootwait quiet` in the configuration.").ok();
            }
            Err(_) => {
                writeln!(
                    d.tx,
                    "error: /config.toml: failed to set bootargs on value `{}`.",
                    bootargs
                )
                .ok();
            }
        }
    } else {
        writeln!(d.tx, "warning: /config.toml: cannot find bootargs on key `configs.bootargs`, using default bootargs in DTB.").ok();
    }

    let root_dir = volume_mgr.open_root_dir(volume0).map_err(|_| ())?;

    // Load firmware.
    let firmware_path = config.configs.firmware.as_ref().map(|s| s.as_str()).unwrap_or_else(|| {
        writeln!(d.tx, "warning: /config.toml: cannot find firmware path on key `configs.firmware`, using default configuration (/zImage).").ok();
        "ZIMAGE"
    });
    let (file_dir, file_name) = locate_file_by_path(&mut d.tx, "firmware", firmware_path);
    let Ok(firmware) = open_file_by_path(&mut volume_mgr, root_dir, file_dir, file_name) else {
        writeln!(
            d.tx,
            "error: /config.toml: file not found for firmware path {}.",
            firmware_path
        )
        .ok();
        return Err(());
    };
    writeln!(
        d.tx,
        "info: /config.toml: firmware located on {}.",
        firmware_path
    )
    .ok();
    let result =
        load_file_into_memory(&mut volume_mgr, firmware, FIRMWARE_ADDRESS, FIRMWARE_LENGTH);
    match result {
        Ok(bytes) => {
            writeln!(
                d.tx,
                "info: load {} success, size = {} bytes",
                file_name, bytes
            )
            .ok();
        }
        Err(Error::FileLength(size)) => {
            writeln!(d.tx, "error: /config.toml: file size for firmware {} is {} bytes, but maximum supported firmware size on the current platform (BL808) is 32,704 KiB.", firmware_path, size).ok();
            return Err(());
        }
        Err(Error::BlockDevice(e)) => {
            writeln!(
                d.tx,
                "error: cannot load file `{}` for underlying block device error: {:?}",
                file_name, e
            )
            .ok();
            return Err(());
        }
        Err(_) => {}
    }

    Ok(OPAQUE_ADDRESS)
}

/// Loads a file from SD card into specified memory address.
// FIXME: should be an unsafe function?
pub fn load_file_into_memory<T: BlockDevice>(
    volume_mgr: &mut VolumeManager<T, MyTimeSource>,
    file: RawFile,
    addr: usize,
    max_size: u32,
) -> Result<usize, Error<T::Error>> {
    // Check file size.
    let file_size = volume_mgr.file_length(file)?;
    if file_size > max_size {
        return Err(Error::FileLength(file_size));
    }

    // Read file content into memory.
    let target = unsafe { core::slice::from_raw_parts_mut(addr as *mut u8, file_size as usize) };
    let size = volume_mgr.read(file, target)?;
    volume_mgr.close_file(file).ok();

    Ok(size)
}

/// Open a file by the given full string slice path.
pub fn open_file_by_path<T: BlockDevice>(
    volume_mgr: &mut VolumeManager<T, MyTimeSource>,
    root_dir: RawDirectory,
    path: &str,
    file_name: &str,
) -> Result<RawFile, ()> {
    // Convert the path to a RawDirectory.
    let directory = path_to_raw_directory(volume_mgr, root_dir, path).map_err(|_| ())?;

    // Find and open the file.
    volume_mgr
        .find_directory_entry(directory, file_name)
        .map_err(|_| ())?;
    let file = volume_mgr
        .open_file_in_dir(directory, file_name, Mode::ReadOnly)
        .map_err(|_| ())?;
    volume_mgr.close_dir(directory).unwrap();

    Ok(file)
}

/// Convert a string slice path to a RawDirectory.
pub fn path_to_raw_directory<T: BlockDevice>(
    volume_mgr: &mut VolumeManager<T, MyTimeSource>,
    root_dir: RawDirectory,
    path: &str,
) -> Result<RawDirectory, ()> {
    // Start from the root directory.
    let mut current_dir = root_dir;

    // Split the path into directories and open each one.
    for dir in path.split('/') {
        if !dir.is_empty() {
            let parent_dir = current_dir;
            current_dir = volume_mgr.open_dir(parent_dir, dir).map_err(|_| ())?;
            volume_mgr.close_dir(parent_dir).unwrap();
        }
    }

    Ok(current_dir)
}

/// Check if a file is in DTB format.
pub fn is_dtb_format<T: BlockDevice>(
    volume_mgr: &mut VolumeManager<T, MyTimeSource>,
    file: RawFile,
) -> bool {
    // Read the first 4 bytes of the file.
    let mut buffer = [0u8; 4];
    if volume_mgr.read(file, &mut buffer).is_err() {
        return false;
    }

    // Check if the first 4 bytes match the DTB magic number.
    let magic_number = u32::from_be_bytes(buffer);
    magic_number == 0xD00DFEED
}

/// Try to locate the file by the given full string slice path.
pub fn locate_file_by_path<'a, W: Write>(
    mut tx: W,
    file_type: &str,
    file_path: &'a str,
) -> (&'a str, &'a str) {
    let (file_dir, file_name);
    if let Some(pos) = file_path.rfind('/') {
        (file_dir, file_name) = file_path.split_at(pos + 1);
        writeln!(
            tx,
            "{} directory: {}, file: {}",
            file_type, file_dir, file_name
        )
        .ok();
    } else {
        (file_dir, file_name) = ("/", file_path);
        writeln!(
            tx,
            "{} directory: {}, file: {}",
            file_type, file_dir, file_name
        )
        .ok();
    }
    (file_dir, file_name)
}
