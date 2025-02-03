#![no_std]
#![no_main]

use bouffalo_hal::{
    prelude::*,
    sdio::{Config as SdhConfig, Sdh},
    uart::Config as UartConfig,
};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_sdmmc::VolumeManager;
use embedded_time::rate::*;
use panic_halt as _;

struct MyTimeSource {}

impl embedded_sdmmc::TimeSource for MyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        // TODO
        embedded_sdmmc::Timestamp::from_calendar(2023, 1, 1, 0, 0, 0).unwrap()
    }
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // Light up led.
    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;
    led.set_state(led_state).ok();

    // Init serial.
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();

    let config = UartConfig::default().set_baudrate(2000000.Bd());
    let mut serial = p
        .uart0
        .freerun(config, ((tx, sig2), (rx, sig3)), &c)
        .unwrap();

    writeln!(serial, "Welcome to sdh-demo!").ok();

    // Sdh gpio init.
    let sdh_clk = p.gpio.io0.into_sdh();
    let sdh_cmd = p.gpio.io1.into_sdh();
    let sdh_d0 = p.gpio.io2.into_sdh();
    let sdh_d1 = p.gpio.io3.into_sdh();
    let sdh_d2 = p.gpio.io4.into_sdh();
    let sdh_d3 = p.gpio.io5.into_sdh();
    let pads = (sdh_clk, sdh_cmd, sdh_d0, sdh_d1, sdh_d2, sdh_d3);

    // Sdh init.
    let config = SdhConfig::default();
    let mut sdcard = Sdh::new(p.sdh, pads, config, &p.glb);
    sdcard.init(&mut serial, true);
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
        riscv::asm::delay(100_000);
    }
}
