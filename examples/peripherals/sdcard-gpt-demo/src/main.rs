#![no_std]
#![no_main]

use bouffalo_hal::{prelude::*, spi::Spi, uart::Config};
use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_hal::spi::MODE_3;
use embedded_sdmmc::*;
use embedded_time::rate::*;
use fatfs::Read;
use gpt_disk_io::Disk;
use gpt_disk_types::{Lba, LbaLe};
use panic_halt as _;

struct MySdCard<T> {
    sdcard: T,
    cache: [u8; 512],
    cache_lba: Lba,
}

impl<T: embedded_sdmmc::BlockDevice> MySdCard<T> {
    #[inline]
    pub fn new(sdcard: T) -> Self {
        MySdCard {
            cache: [0; 512],
            cache_lba: Lba(sdcard.num_blocks().unwrap().0.into()),
            sdcard,
        }
    }
    #[inline]
    fn read_block(&mut self, lba: Lba, buf: &mut Block) -> Result<(), Error> {
        if self.cache_lba != lba {
            let mut block = [Block::new()];
            let block_idx = BlockIdx(lba.0.try_into().unwrap());
            self.sdcard.read(&mut block, block_idx, "").unwrap();
            self.cache.copy_from_slice(block[0].as_slice());
            self.cache_lba = lba;
        }
        buf.copy_from_slice(&self.cache);
        Ok(())
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Other,
}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Other => write!(f, "Error"),
        }
    }
}

impl<T: embedded_sdmmc::BlockDevice> gpt_disk_io::BlockIo for MySdCard<T> {
    type Error = Error;
    #[inline]
    fn block_size(&self) -> gpt_disk_io::gpt_disk_types::BlockSize {
        gpt_disk_io::gpt_disk_types::BlockSize::new(512).unwrap()
    }
    #[inline]
    fn read_blocks(&mut self, lba: Lba, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut block = Block::new();
        let mut lba = lba;
        for b in buf.chunks_mut(512) {
            self.read_block(lba, &mut block).unwrap();
            b.copy_from_slice(block.as_slice());
            lba.0 += 1;
        }
        Ok(())
    }
    #[inline]
    fn write_blocks(&mut self, _lba: Lba, _buf: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }
    #[inline]
    fn num_blocks(&mut self) -> Result<u64, Self::Error> {
        Ok(self.sdcard.num_blocks().unwrap().0.into())
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct MyVolume<T> {
    my_sdcard: MySdCard<T>,
    begin: u64,
    size: u64,
    offset: u64,
}

impl<T: embedded_sdmmc::BlockDevice> MyVolume<T> {
    #[inline]
    pub fn new(my_sdcard: MySdCard<T>, begin: Lba, end: Lba) -> Self {
        MyVolume {
            my_sdcard,
            begin: begin.0 * 512,
            size: end.0 * 512 - begin.0 * 512,
            offset: 0,
        }
    }
    #[inline]
    fn to_lba(&self, offset: u64) -> (Lba, u64) {
        (
            Lba((self.begin + offset) / 512),
            (self.begin + offset) % 512,
        )
    }
}

impl fatfs::IoError for Error {
    #[inline]
    fn is_interrupted(&self) -> bool {
        false
    }
    #[inline]
    fn new_unexpected_eof_error() -> Self {
        Self::Other
    }
    #[inline]
    fn new_write_zero_error() -> Self {
        Self::Other
    }
}

impl<T: embedded_sdmmc::BlockDevice> fatfs::IoBase for MyVolume<T> {
    type Error = Error;
}

impl<T: embedded_sdmmc::BlockDevice> fatfs::Read for MyVolume<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut bytes_read = 0;
        let mut block = Block::new();
        for b in buf {
            let (lba, offset_in_block) = self.to_lba(self.offset);
            self.my_sdcard.read_block(lba, &mut block).unwrap();
            *b = block[offset_in_block as usize];
            bytes_read += 1;
            self.offset += 1;
        }
        Ok(bytes_read)
    }
}

impl<T: embedded_sdmmc::BlockDevice> fatfs::Write for MyVolume<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // TODO
        Ok(buf.len())
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        // TODO
        Ok(())
    }
}

impl<T: embedded_sdmmc::BlockDevice> fatfs::Seek for MyVolume<T> {
    #[inline]
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, Self::Error> {
        let new_offset_opt: Option<u64> = match pos {
            fatfs::SeekFrom::Current(x) => i64::try_from(self.offset)
                .ok()
                .and_then(|n| n.checked_add(x))
                .and_then(|n| u64::try_from(n).ok()),
            fatfs::SeekFrom::Start(x) => Some(x),
            fatfs::SeekFrom::End(o) => i64::try_from(self.size)
                .ok()
                .and_then(|size| size.checked_add(o))
                .and_then(|n| u64::try_from(n).ok()),
        };
        if let Some(new_offset) = new_offset_opt {
            if new_offset > self.size {
                Err(Error::Other)
            } else {
                self.offset = new_offset;
                Ok(self.offset)
            }
        } else {
            Err(Error::Other)
        }
    }
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();
    let pads = ((tx, sig2), (rx, sig3));

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, pads, &c).unwrap();
    writeln!(serial, "Hello world!").ok();

    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::High;

    let spi_clk = p.gpio.io3.into_spi::<1>();
    let spi_mosi = p.gpio.io1.into_spi::<1>();
    let spi_miso = p.gpio.io2.into_spi::<1>();
    let spi_cs = p.gpio.io0.into_spi::<1>();

    let spi_sd = Spi::new(
        p.spi1,
        (spi_clk, spi_mosi, spi_miso, spi_cs),
        MODE_3,
        &p.glb,
    );

    let delay = riscv::delay::McycleDelay::new(40_000_000);
    let sdcard = SdCard::new(spi_sd, delay);
    while sdcard.get_card_type().is_none() {
        core::hint::spin_loop();
    }

    writeln!(serial, "Card size: {}", sdcard.num_bytes().unwrap()).ok();
    writeln!(serial, "").ok();

    let my_sdcard = MySdCard::new(sdcard);
    let mut disk = Disk::new(my_sdcard).unwrap();
    let mut buffer = [0u8; 512];
    let header = disk.read_primary_gpt_header(&mut buffer).unwrap();
    writeln!(serial, "Primary header: {:?}", header).ok();

    let layout = header.get_partition_entry_array_layout().unwrap();
    writeln!(serial, "Layout: {:?}", layout).ok();
    let mut iter = disk
        .gpt_partition_entry_array_iter(layout, &mut buffer)
        .unwrap();
    while let Some(res) = iter.next() {
        let res = res.unwrap();
        if res.starting_lba == LbaLe(gpt_disk_types::U64Le([0; 8])) {
            break;
        }
        writeln!(serial, "Partition: {:?}", res).ok();
    }
    drop(iter);

    // TODO: switch to disk.into_inner() once Disk struct have this function
    let my_sdcard: MySdCard<
        SdCard<
            Spi<
                bouffalo_rt::soc::bl808::SPI0,
                (
                    bouffalo_hal::gpio::Alternate<
                        bouffalo_rt::soc::bl808::GLBv2,
                        3,
                        bouffalo_hal::gpio::Spi<1>,
                    >,
                    bouffalo_hal::gpio::Alternate<
                        bouffalo_rt::soc::bl808::GLBv2,
                        1,
                        bouffalo_hal::gpio::Spi<1>,
                    >,
                    bouffalo_hal::gpio::Alternate<
                        bouffalo_rt::soc::bl808::GLBv2,
                        2,
                        bouffalo_hal::gpio::Spi<1>,
                    >,
                    bouffalo_hal::gpio::Alternate<
                        bouffalo_rt::soc::bl808::GLBv2,
                        0,
                        bouffalo_hal::gpio::Spi<1>,
                    >,
                ),
                1,
            >,
            riscv::delay::McycleDelay,
        >,
    > = unsafe { core::mem::transmute(disk) };
    let my_volume = MyVolume::new(my_sdcard, Lba(2048), Lba(616447));
    let fs = fatfs::FileSystem::new(my_volume, fatfs::FsOptions::new()).unwrap();
    let root_dir = fs.root_dir();
    writeln!(serial, "List files:").ok();
    for r in root_dir.iter() {
        let entry = r.unwrap();
        write!(serial, "  File: ").ok();
        for b in entry.short_file_name_as_bytes() {
            write!(serial, "{}", *b as char).ok();
        }
        writeln!(serial, "").ok();
    }
    writeln!(serial, "").ok();

    writeln!(serial, "Read a file: ").ok();
    let mut file = root_dir.open_file("TEST").unwrap();
    let mut size = 0u64;
    let mut buffer = [0u8];
    while let Ok(s) = file.read(&mut buffer) {
        if s != 1 {
            break;
        }
        write!(serial, "{}", buffer[0] as char).ok();
        size += 1;
    }
    writeln!(serial, "").ok();
    writeln!(serial, "File size: {}", size).ok();

    writeln!(serial, "OK").ok();

    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        riscv::asm::delay(100_000)
    }
}
