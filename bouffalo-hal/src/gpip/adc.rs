use super::register::*;
use crate::hbn;
use cfg_if::cfg_if;
use core::ops::Deref;

/// Generic ADC implementation for the GPIP peripheral.
impl<G: Deref<Target = RegisterBlock>> Gpip<G> {
    /// Calibrate adc (if adc is already calibrated, nothing will be done).
    #[inline]
    pub fn adc_calibrate(&mut self) {}

    /// Feature control.
    #[inline]
    pub fn adc_feature_control(
        &mut self,
        cmd: AdcCommmand,
        vbat_en: bool,
        hbn: &hbn::RegisterBlock,
    ) {
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
                hbn.gpadc_config_2.modify(|val| {
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
    pub fn adc_channel_config(&mut self, channels: &[AdcChannels], hbn: &hbn::RegisterBlock) {
        if self.adc_config.is_none() {
            panic!("ADC config not set");
        }

        if !hbn.gpadc_config_1.read().is_scan_enabled() {
            if channels.len() > 1 {
                panic!("Too many adc channels.");
            }

            if channels.is_empty() {
                panic!("No channels provided");
            }

            unsafe {
                hbn.gpadc_command.modify(|val| {
                    val.set_pos_sel(channels[0].pos_ch)
                        .set_neg_sel(channels[0].neg_ch)
                });
            }
        } else {
            // Scan mode: multiple channels can be configured (up to 12)
            if channels.len() > 12 {
                panic!("Too many channels for scan mode (max 12)");
            }

            if channels.is_empty() {
                panic!("No channels provided");
            }

            if channels.len() <= 6 {
                // First 6 channels: use sequence_1 and sequence_3 specific methods
                unsafe {
                    hbn.gpadc_converation_sequence_1.modify(|mut seq1| {
                        hbn.gpadc_converation_sequence_3.modify(|mut seq3| {
                            for (i, channel) in channels.iter().enumerate() {
                                match i {
                                    0 => {
                                        seq1 = seq1.set_scan_pos_0(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_0(channel.neg_ch);
                                    }
                                    1 => {
                                        seq1 = seq1.set_scan_pos_1(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_1(channel.neg_ch);
                                    }
                                    2 => {
                                        seq1 = seq1.set_scan_pos_2(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_2(channel.neg_ch);
                                    }
                                    3 => {
                                        seq1 = seq1.set_scan_pos_3(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_3(channel.neg_ch);
                                    }
                                    4 => {
                                        seq1 = seq1.set_scan_pos_4(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_4(channel.neg_ch);
                                    }
                                    5 => {
                                        seq1 = seq1.set_scan_pos_5(channel.pos_ch);
                                        seq3 = seq3.set_scan_neg_5(channel.neg_ch);
                                    }
                                    _ => unreachable!("Channel count already checked"),
                                }
                            }
                            seq3
                        });
                        seq1
                    });
                }
            } else {
                // More than 6 channels: need to use two sequence register groups
                unsafe {
                    hbn.gpadc_converation_sequence_1.modify(|mut seq1| {
                        hbn.gpadc_converation_sequence_2.modify(|mut seq2| {
                            hbn.gpadc_converation_sequence_3.modify(|mut seq3| {
                                hbn.gpadc_converation_sequence_4.modify(|mut seq4| {
                                    // First 6 channels
                                    for i in 0..6 {
                                        let channel = &channels[i];
                                        match i {
                                            0 => {
                                                seq1 = seq1.set_scan_pos_0(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_0(channel.neg_ch);
                                            }
                                            1 => {
                                                seq1 = seq1.set_scan_pos_1(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_1(channel.neg_ch);
                                            }
                                            2 => {
                                                seq1 = seq1.set_scan_pos_2(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_2(channel.neg_ch);
                                            }
                                            3 => {
                                                seq1 = seq1.set_scan_pos_3(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_3(channel.neg_ch);
                                            }
                                            4 => {
                                                seq1 = seq1.set_scan_pos_4(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_4(channel.neg_ch);
                                            }
                                            5 => {
                                                seq1 = seq1.set_scan_pos_5(channel.pos_ch);
                                                seq3 = seq3.set_scan_neg_5(channel.neg_ch);
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    // Remaining channels (6-11)
                                    for i in 6..channels.len() {
                                        let channel = &channels[i];
                                        match i - 6 {
                                            0 => {
                                                seq2 = seq2.set_scan_pos_6(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_6(channel.neg_ch);
                                            }
                                            1 => {
                                                seq2 = seq2.set_scan_pos_7(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_7(channel.neg_ch);
                                            }
                                            2 => {
                                                seq2 = seq2.set_scan_pos_8(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_8(channel.neg_ch);
                                            }
                                            3 => {
                                                seq2 = seq2.set_scan_pos_9(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_9(channel.neg_ch);
                                            }
                                            4 => {
                                                seq2 = seq2.set_scan_pos_10(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_10(channel.neg_ch);
                                            }
                                            5 => {
                                                seq2 = seq2.set_scan_pos_11(channel.pos_ch);
                                                seq4 = seq4.set_scan_neg_11(channel.neg_ch);
                                            }
                                            _ => unreachable!("Up to 12 channels"),
                                        }
                                    }
                                    seq4
                                });
                                seq3
                            });
                            seq2
                        });
                        seq1
                    });
                }
            }

            // Set scan length
            unsafe {
                hbn.gpadc_config_1
                    .modify(|val| val.set_scan_length((channels.len() - 1) as u8));
            }
        }
    }

    /// Start adc coversion.
    #[inline]
    pub fn adc_start_conversion(&mut self, hbn: &hbn::RegisterBlock) {
        unsafe {
            hbn.gpadc_command.modify(|val| val.stop_conversion());

            sleep(100);

            hbn.gpadc_command.modify(|val| val.start_conversion());
        }
    }

    /// Stop adc coversion.
    #[inline]
    pub fn adc_stop_conversion(&mut self, hbn: &hbn::RegisterBlock) {
        unsafe {
            hbn.gpadc_command.modify(|val| val.stop_conversion());
        }
    }

    /// Init internal temperature sensor.
    #[inline]
    pub fn adc_tsen_init(&mut self, external_ts: bool, hbn: &hbn::RegisterBlock) {
        unsafe {
            hbn.gpadc_command.modify(|val| {
                let val = val.disable_dwa().disable_chip_sen_pu();
                cfg_if! {
                    if #[cfg(any(feature = "bl702", feature = "bl602"))] {
                        val.disable_sensor_test_v1().set_sensor_v1(0)
                    } else {
                        val.disable_sensor_test_v2().set_sensor_v2(0)
                    }
                }
            });
            hbn.gpadc_config_2.modify(|val| {
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
            hbn.gpadc_config_1.modify(|val| val.disable_dither());
            hbn.gpadc_command.modify(|val| val.enable_mic2_diff());
        }
    }

    /// Get number of adc completed conversions.
    #[inline]
    pub fn adc_get_complete_num(&self) -> u16 {
        self.gpip.gpadc_config.read().fifo_data_count()
    }

    /// Get adc raw data.
    #[inline]
    pub fn adc_get_raw_data(&self) -> u32 {
        self.gpip.gpadc_dma_rdata.read().dma_rdata()
    }

    /// Get internal temperature sensor's temperature.
    pub fn adc_get_tsen_temp(&mut self, hbn: &hbn::RegisterBlock) -> f32 {
        const TSEN_OFFSET: f32 = 2042.0;
        unsafe {
            self.gpip.gpadc_config.modify(|val| {
                val.set_fifo_threshold(GpadcFifoThreshold::OneData)
                    .clear_fifo()
            });
        }

        unsafe {
            hbn.gpadc_config_2.modify(|val| val.disable_tsvbe_low());
        }

        self.adc_start_conversion(hbn);

        let mut sum0 = 0u32;
        let sample_count = 16;

        for _ in 0..8 {
            while self.adc_get_complete_num() == 0 {
                core::hint::spin_loop();
            }
            let _ = self.adc_get_raw_data();
        }

        for _ in 0..sample_count {
            while self.adc_get_complete_num() == 0 {
                core::hint::spin_loop();
            }
            sum0 += self.adc_get_raw_data() & 0xFFFF;
        }

        self.adc_stop_conversion(hbn);
        let v0 = (sum0 + 8) / 16;

        unsafe {
            self.gpip.gpadc_config.modify(|val| val.clear_fifo());
            hbn.gpadc_config_2.modify(|val| val.enable_tsvbe_low());
        }

        self.adc_start_conversion(hbn);

        let mut sum1 = 0u32;

        for _ in 0..8 {
            while self.adc_get_complete_num() == 0 {
                core::hint::spin_loop();
            }
            let _ = self.adc_get_raw_data();
        }

        for _ in 0..sample_count {
            while self.adc_get_complete_num() == 0 {
                core::hint::spin_loop();
            }
            sum1 += self.adc_get_raw_data() & 0xFFFF;
        }

        self.adc_stop_conversion(hbn);
        let v1 = (sum1 + 8) / 16;

        if v0 > v1 {
            ((v0 as f32 - v1 as f32) - TSEN_OFFSET) / 7.753
        } else {
            0.0
        }
    }
}

fn sleep(n: u32) {
    for _ in 0..n * 125 {
        unsafe { core::arch::asm!("nop") }
    }
}
