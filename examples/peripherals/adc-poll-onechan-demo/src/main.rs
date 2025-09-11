#![no_std]
#![no_main]

use bouffalo_hal::{
    efuse::Efuse,
    gpip::{AdcChannels, AdcConfig, AdcResult, Gpip},
    hbn::{GpadcChannel, GpadcVref},
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

    writeln!(serial, "Welcome to ADC poll one channel demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(AdcConfig::default().set_vref(GpadcVref::V3p2)),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    let chan = &[AdcChannels {
        pos_ch: GpadcChannel::Channel0,
        neg_ch: GpadcChannel::ChannelVGND,
    }];

    gpip.adc_channel_config(chan, &p.hbn);
    gpip.adc_start_conversion(&p.hbn);

    let mut raw_data = 0u32;
    for i in 0..26 {
        while (gpip.adc_get_complete_num() as usize) == 0 {
            core::hint::spin_loop();
        }

        if i > 9 {
            raw_data += gpip.adc_get_raw_data();
        } else {
            // Discard initial 10 samples
            let _ = gpip.adc_get_raw_data();
        }
    }

    raw_data /= 16;

    gpip.adc_stop_conversion(&p.hbn);

    let res = &mut [AdcResult {
        pos_chan: None,
        neg_chan: None,
        value: 0,
        millivolt: 0,
    }; 1];

    gpip.adc_parse_result(&[raw_data], res, &p.hbn);

    writeln!(serial, "Raw data: 0x{:08X}", raw_data).ok();
    writeln!(
        serial,
        "Channel {:?} value = 0x{:08X}, millivolt = {}mv.",
        res[0].pos_chan.unwrap(),
        res[0].value,
        res[0].millivolt
    )
    .ok();

    loop {}
}

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 100 {
            core::arch::asm!("nop");
        }
    }
}
