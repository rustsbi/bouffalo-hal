use super::register::*;
use cfg_if::cfg_if;
use core::ops::Deref;

/// Generic ADC implementation for the GPIP peripheral.
impl<G: Deref<Target = RegisterBlock>> Gpip<G> {
    /// Calibrate adc (if adc is already calibrated, nothing will be done).
    #[inline]
    pub fn adc_calibrate(&mut self) {}

    /// Feature control.
    #[inline]
    pub fn adc_feature_control(&mut self, cmd: AdcCommmand, vbat_en: bool) {
        match cmd {
            AdcCommmand::ClearFifo => unsafe {
                self.gpip
                    .gpadc_config
                    .modify(|val| val.disable_dma().clear_fifo());
                self.gpip.gpadc_config.modify(|val| {
                    if self.adc_config.unwrap().dma_en {
                        val.enable_dma()
                    } else {
                        val.disable_dma()
                    }
                });
            },
            AdcCommmand::VbatEn => unsafe {
                self.gpip.gpadc_config_2.modify(|val| {
                    if vbat_en {
                        val.enable_vbat()
                    } else {
                        val.disable_vbat()
                    }
                });
            },
        }
    }

    /// Configure adc channels.
    #[inline]
    pub fn adc_channel_config(&mut self, channels: &[AdcChannels]) {
        if !self.adc_config.unwrap().scan_en {
            if channels.len() > 1 {
                panic!("Too many adc channels.")
            }

            unsafe {
                self.gpip.gpadc_command.modify(|val| {
                    val.set_pos_sel(channels[0].pos_ch)
                        .set_neg_sel(channels[0].neg_ch)
                });
            }
        }
    }

    /// Start adc coversion.
    #[inline]
    pub fn adc_start_conversion(&mut self) {
        unsafe {
            self.gpip.gpadc_command.modify(|val| val.stop_conversion());

            sleep(100);

            self.gpip.gpadc_command.modify(|val| val.start_conversion());
        }
    }

    /// Stop adc coversion.
    #[inline]
    pub fn adc_stop_conversion(&mut self) {
        unsafe {
            self.gpip.gpadc_command.modify(|val| val.stop_conversion());
        }
    }

    /// Init internal temperature sensor.
    #[inline]
    pub fn adc_tsen_init(&mut self, external_ts: bool) {
        unsafe {
            self.gpip.gpadc_command.modify(|val| {
                let val = val.disable_dwa().disable_chip_sen_pu();
                cfg_if! {
                    if #[cfg(any(feature = "bl702", feature = "bl602"))] {
                        val.disable_sensor_test_v1().set_sensor_v1(0)
                    } else {
                        val.disable_sensor_test_v2().set_sensor_v2(0)
                    }
                }
            });
            self.gpip.gpadc_config_2.modify(|val| {
                val.disable_tsvbe_low()
                    .disable_test()
                    .set_test_sel(0)
                    .disable_pga_vcmi()
                    .set_chop_mode(1)
                    .set_dly_sel(2)
                    .enable_ts()
                    .set_tsext_sel(external_ts)
                    .set_pga_vcm(1)
                    .set_pga_os_cal(0)
            });
            self.gpip.gpadc_config_1.modify(|val| val.disable_dither());
            self.gpip.gpadc_command.modify(|val| val.enable_mic2_diff());
        }
    }
}

fn sleep(n: u32) {
    for _ in 0..n * 125 {
        unsafe { core::arch::asm!("nop") }
    }
}
