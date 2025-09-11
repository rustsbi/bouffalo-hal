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

    writeln!(serial, "Welcome to ADC poll diff demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(
            AdcConfig::default()
                .enable_diff_mode()
                .enable_scan()
                .disable_continuous_conv()
                .set_vref(GpadcVref::V3p2),
        ),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    let chans = &[
        AdcChannels {
            pos_ch: GpadcChannel::Channel0,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::ChannelVGND,
            neg_ch: GpadcChannel::Channel2,
        },
    ];

    gpip.adc_channel_config(chans, &p.hbn);

    for _ in 0..10 {
        gpip.adc_start_conversion(&p.hbn);

        while (gpip.adc_get_complete_num() as usize) < chans.len() {
            core::hint::spin_loop();
        }

        for _ in 0..chans.len() {
            let raw_data = gpip.adc_get_raw_data();
            writeln!(serial, "Raw data: 0x{:08X}", raw_data);
            let res = &mut [AdcResult {
                pos_chan: None,
                neg_chan: None,
                value: 0,
                millivolt: 0,
            }; 1];
            gpip.adc_parse_result(&[raw_data], res, &p.hbn);
            writeln!(
                serial,
                "PosChannel {:?} vs NegChannel {:?} value = 0x{:08X}, millivolt = {}mv.",
                res[0].pos_chan.unwrap(),
                res[0].neg_chan.unwrap(),
                res[0].value,
                res[0].millivolt
            );
        }
        gpip.adc_stop_conversion(&p.hbn);
        delay(100);
    }

    loop {}
}

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 100 {
            core::arch::asm!("nop");
        }
    }
}
