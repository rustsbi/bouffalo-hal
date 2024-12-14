#![no_std]
#![no_main]

use core::{arch::asm, ptr};

use bouffalo_hal::{prelude::*, uart::Config};
use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx, VolumeManager};
use embedded_time::rate::*;
use panic_halt as _;
use sdh::*;

mod sdh;

struct MyTimeSource {}

impl embedded_sdmmc::TimeSource for MyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        // TODO
        embedded_sdmmc::Timestamp::from_calendar(2023, 1, 1, 0, 0, 0).unwrap()
    }
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // light up led
    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;
    led.set_state(led_state).ok();

    // init serial
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, ((tx, sig2), (rx, sig3)), &c);

    writeln!(serial, "Welcome to sdh-demo!").ok();

    // sdh gpio init
    p.gpio.io0.into_sdh();
    p.gpio.io1.into_sdh();
    p.gpio.io2.into_sdh();
    p.gpio.io3.into_sdh();
    p.gpio.io4.into_sdh();
    p.gpio.io5.into_sdh();

    // sdh init
    let sdcard = sdh_init(&mut serial);
    let time_source = MyTimeSource {};
    let mut volume_mgr = VolumeManager::new(sdcard, time_source);
    let volume_res = volume_mgr.open_raw_volume(embedded_sdmmc::VolumeIdx(0));
    if let Err(e) = volume_res {
        writeln!(serial, "Failed to open volume: {:?}", e).ok();
        loop {}
    }
    let volume0 = volume_res.unwrap();
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
        sleep_ms(1000);
    }
}

#[inline]
pub(crate) fn set_bits(val: u32, pos: u32, len: u32, val_in: u32) -> u32 {
    let mask = ((1 << len) - 1) << pos;
    (val & !mask) | ((val_in << pos) & mask)
}

#[inline]
pub(crate) fn is_bit_set(val: u32, pos: u32) -> bool {
    (val & (1 << pos)) != 0
}

#[inline]
pub(crate) fn sleep_ms(n: u32) {
    for _ in 0..n * 125 {
        unsafe { asm!("nop") }
    }
}
