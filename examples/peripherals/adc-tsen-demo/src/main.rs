#![no_std]
#![no_main]

use bouffalo_hal::{
    efuse::Efuse,
    gpip::{AdcChannels, AdcConfig, Gpip},
    hbn::GpadcChannel,
    prelude::*,
    uart::Config,
};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();

    let mut gpip = Gpip::new(p.gpip, Some(AdcConfig::default()), None, &p.glb, &p.hbn);

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    let chans = AdcChannels {
        pos_ch: GpadcChannel::ChannelTSENP,
        neg_ch: GpadcChannel::ChannelVGND,
    };

    gpip.adc_channel_config(&[chans], &p.hbn);
    gpip.adc_tsen_init(false, &p.hbn);

    writeln!(serial, "Welcome to ADC internal temperature sensor demo!").ok();

    for _ in 0..5 {
        delay(100);
        let temp = gpip.adc_get_tsen_temp(&p.hbn) as u32;
        writeln!(serial, "Current temperature = {}.", temp).ok();
    }

    writeln!(serial, "Finished").ok();

    loop {}
}

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 100 {
            core::arch::asm!("nop");
        }
    }
}
