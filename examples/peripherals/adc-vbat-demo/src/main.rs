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

    writeln!(serial, "Welcome to ADC vbat demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(
            AdcConfig::default()
                .set_vref(GpadcVref::V3p2)
                .disable_continuous_conv(),
        ),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    let chans = AdcChannels {
        pos_ch: GpadcChannel::ChannelVBatHalf,
        neg_ch: GpadcChannel::ChannelVGND,
    };

    gpip.adc_channel_config(&[chans], &p.hbn);
    gpip.adc_feature_control(AdcCommand::VbatEn, true, &p.hbn);

    let res = &mut [AdcResult {
        pos_chan: None,
        neg_chan: None,
        value: 0,
        millivolt: 0,
    }];

    for _ in 0..10 {
        gpip.adc_start_conversion(&p.hbn);

        while (gpip.adc_get_complete_num() as usize) == 0 {
            core::hint::spin_loop();
        }

        let raw_data = gpip.adc_get_raw_data();
        gpip.adc_parse_result(&[raw_data], res, &p.hbn);

        writeln!(serial, "VBAT: {} mV", res[0].millivolt * 2).ok(); // Vbat is divided by 2 internally

        gpip.adc_stop_conversion(&p.hbn);
        delay(500);
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
