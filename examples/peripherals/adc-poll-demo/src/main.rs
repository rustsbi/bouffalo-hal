#![no_std]
#![no_main]

use bouffalo_hal::{
    efuse::Efuse,
    gpip::{AdcChannels, AdcCommand, AdcConfig, AdcResult, Gpip},
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

    writeln!(serial, "Welcome to ADC poll demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(AdcConfig::default().set_vref(GpadcVref::V3p2)),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    let chans = [
        AdcChannels {
            pos_ch: GpadcChannel::Channel0,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel1,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel2,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel3,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel4,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel5,
            neg_ch: GpadcChannel::ChannelVGND,
        },
    ];

    for chan in chans {
        gpip.adc_feature_control(AdcCommand::ClearFifo, false, &p.hbn);
        gpip.adc_channel_config(&[chan], &p.hbn);
        gpip.adc_start_conversion(&p.hbn);

        let value = &mut [0u32; 26];

        for i in 0..26 {
            while gpip.adc_get_complete_num() == 0 {
                core::hint::spin_loop();
            }
            value[i] = gpip.adc_get_raw_data();
        }

        let result = &mut [AdcResult {
            pos_chan: Some(chan.pos_ch),
            neg_chan: Some(chan.neg_ch),
            value: 0,
            millivolt: 0,
        }; 26];

        gpip.adc_parse_result(value, result, &p.hbn);

        for res in result.iter().skip(10) {
            writeln!(
                serial,
                "Channel {:?} value = 0x{:08X}, millivolt = {}mv.",
                res.pos_chan.unwrap(),
                res.value,
                res.millivolt
            )
            .ok();
        }
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
