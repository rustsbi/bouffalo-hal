// This is a example of how to read a SD card with a MBR partition table and a FAT32 partition.
// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p sdcard-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{clocks::Clocks, gpio::Pads, prelude::*, spi::Spi, uart::UartMuxes, GLB, SPI, UART};
use embedded_hal::spi::MODE_3;
use embedded_sdmmc::{SdCard, VolumeManager};
use embedded_time::rate::*;
use panic_halt as _;

struct MyTimeSourse {}

impl embedded_sdmmc::TimeSource for MyTimeSourse {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        // TODO
        embedded_sdmmc::Timestamp::from_calendar(2023, 1, 1, 0, 0, 0).unwrap()
    }
}

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let uart0: UART<Static<0x2000A000>, 0> = unsafe { core::mem::transmute(()) };
    let uart_muxes: UartMuxes<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {
        xtal: Hertz(40_000_000),
    };
    let gpio: Pads<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let spi: SPI<Static<0x30008000>> = unsafe { core::mem::transmute(()) };

    let tx = gpio.io14.into_uart();
    let rx = gpio.io15.into_uart();
    let sig2 = uart_muxes.sig2.into_transmit::<0>();
    let sig3 = uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let mut serial = uart0.freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &clocks);

    let mut led = gpio.io8.into_floating_output();
    let mut led_state = PinState::High;

    let spi_clk = gpio.io3.into_spi::<1>();
    let spi_mosi = gpio.io1.into_spi::<1>();
    let spi_miso = gpio.io2.into_spi::<1>();
    let spi_cs = gpio.io0.into_spi::<1>();
    let spi_sd = Spi::new(spi, (spi_clk, spi_mosi, spi_miso, spi_cs), MODE_3, &glb);

    let delay = riscv::delay::McycleDelay::new(40_000_000);
    // TODO: let embedded_sdmmc::SdCard control cs pin
    let fake_cs = gpio.io12.into_floating_output();
    let sdcard = SdCard::new(spi_sd, fake_cs, delay);
    writeln!(serial, "Card size: {}", sdcard.num_bytes().unwrap()).ok();

    let time_source = MyTimeSourse {};
    let mut volume_mgr = VolumeManager::new(sdcard, time_source);

    let volume0 = volume_mgr
        .open_volume(embedded_sdmmc::VolumeIdx(0))
        .unwrap();
    let root_dir = volume_mgr.open_root_dir(volume0).unwrap();

    volume_mgr
        .iterate_dir(root_dir, |entry| {
            writeln!(serial, "Entry: {:?}", entry).ok();
        })
        .unwrap();

    volume_mgr.close_dir(root_dir).unwrap();

    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        unsafe { riscv::asm::delay(100_000) }
    }
}
