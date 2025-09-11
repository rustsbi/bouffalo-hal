use super::register::*;
use crate::{
    efuse, glb,
    hbn::{self, GpadcChannel},
};
use cfg_if::cfg_if;
use core::ops::Deref;

/// ADC conversion result structure
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdcResult {
    pub pos_chan: Option<GpadcChannel>,
    pub neg_chan: Option<GpadcChannel>,
    pub value: u32,
    pub millivolt: i32,
}

/// Generic ADC implementation for the GPIP peripheral.
impl<G: Deref<Target = RegisterBlock>> Gpip<G> {
    /// Calibrate adc (if adc is already calibrated, nothing will be done).
    #[inline]
    pub fn adc_calibrate(
        &mut self,
        efuse: &efuse::Efuse<impl Deref<Target = efuse::RegisterBlock>>,
        hbn: &hbn::RegisterBlock,
        glb: Option<&glb::v1::RegisterBlock>,
    ) {
        if !self.adc_calibration_complete {
            self.adc_coe = efuse.get_adc_trim(glb, Some(hbn));
            self.adc_update_trim(hbn);
            self.tsen_offset = efuse.get_adc_tsen_trim(glb, Some(hbn));
            self.adc_calibration_complete = true;
        }
    }

    /// Feature control.
    #[inline]
    pub fn adc_feature_control(
        &mut self,
        cmd: AdcCommand,
        vbat_en: bool,
        hbn: &hbn::RegisterBlock,
    ) {
        match cmd {
            AdcCommand::ClearFifo => unsafe {
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
            AdcCommand::VbatEn => unsafe {
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

    /// Enable adc interrupt.
    #[inline]
    pub fn enable_interrupt(&mut self, enable: bool) {
        unsafe {
            if enable {
                self.gpip.gpadc_config.modify(|val| val.enable_adc_ready());
            } else {
                self.gpip.gpadc_config.modify(|val| val.disable_adc_ready());
            }
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

    /// Set reference channel for differential measurements.
    #[inline]
    pub fn adc_set_reference_channel(&mut self, channel: GpadcChannel, millivolt: i32) {
        self.adc_reference_channel = Some(channel);
        self.adc_reference_mv = millivolt;
    }

    /// Update ADC calibration values based on measurements
    #[inline]
    pub fn adc_update_trim(&mut self, hbn: &hbn::RegisterBlock) {
        if self.adc_config.is_none() {
            return;
        }

        let user_config = self.adc_config.unwrap();

        // Set continuous conversion, disable scan
        unsafe {
            hbn.gpadc_config_1
                .modify(|val| val.enable_continuous_conv().disable_scan());
        }

        // Enable differential mode and VBAT
        unsafe {
            hbn.gpadc_config_2
                .modify(|val| val.enable_diff_mode().enable_vbat());
        }

        // Set channels to VBAT_HALF
        unsafe {
            hbn.gpadc_command.modify(|val| {
                val.unset_neg_gnd()
                    .set_pos_sel(GpadcChannel::ChannelVBatHalf)
                    .set_neg_sel(GpadcChannel::ChannelVBatHalf)
            });
        }

        // Start conversion
        self.adc_start_conversion(hbn);

        let mut os_val = 0u32;
        let mut neg = false;

        // Process 10 samples, discard first 5, use last 5
        for i in 0..10 {
            unsafe {
                self.gpip.gpadc_config.modify(|val| val.clear_fifo());
            }

            while self.adc_get_complete_num() == 0 {}

            let mut raw_val = self.adc_get_raw_data();

            // Only process samples 5-9
            if i > 4 {
                if (raw_val & 0x8000) != 0 {
                    raw_val = !raw_val;
                    raw_val = raw_val.wrapping_add(1);
                    neg = true;
                }

                os_val += raw_val & 0xffff;
            }
        }

        // Stop conversion
        self.adc_stop_conversion(hbn);
        sleep(10);

        let os_val_div5 = (os_val / 5) as i32;

        if neg {
            self.adc_os2 = os_val_div5 * 2 - os_val_div5 * 4;
        } else {
            self.adc_os2 = os_val_div5 * 2;
        }

        // Update coefficient
        self.adc_coe = self.adc_coe - (self.adc_os2 as f32) / 40960.0;

        // Second phase: single-ended mode, disable VBAT
        os_val = 0;
        unsafe {
            hbn.gpadc_command.modify(|val| val.set_neg_gnd());
            hbn.gpadc_config_2
                .modify(|val| val.disable_diff_mode().disable_vbat());
        }

        // Set both channels to GND
        unsafe {
            hbn.gpadc_command.modify(|val| {
                val.set_pos_sel(GpadcChannel::ChannelVGND)
                    .set_neg_sel(GpadcChannel::ChannelVGND)
            });
        }

        // Second round of conversion
        self.adc_start_conversion(hbn);

        for i in 0..10 {
            unsafe {
                self.gpip.gpadc_config.modify(|val| val.clear_fifo());
            }

            while self.adc_get_complete_num() == 0 {}

            let raw_val = self.adc_get_raw_data();

            // Only process samples 5-9
            if i > 4 {
                os_val += raw_val & 0xffff;
            }
        }

        // Stop conversion
        self.adc_stop_conversion(hbn);
        sleep(10);

        // Calculate os1
        if os_val > 0 {
            self.adc_os1 = os_val / 5;
        } else {
            self.adc_os1 = 0;
        }

        // Restore user configuration
        unsafe {
            hbn.gpadc_config_1.modify(|val| {
                if user_config.continuous_conv_en {
                    val.enable_continuous_conv()
                } else {
                    val.disable_continuous_conv()
                }
            });

            hbn.gpadc_config_1.modify(|val| {
                if user_config.scan_en {
                    val.enable_scan()
                } else {
                    val.disable_scan()
                }
            });

            hbn.gpadc_config_2.modify(|val| {
                if user_config.diff_en {
                    val.enable_diff_mode()
                } else {
                    val.disable_diff_mode()
                }
            });

            hbn.gpadc_command.modify(|val| {
                if user_config.diff_en {
                    val.unset_neg_gnd()
                } else {
                    val.set_neg_gnd()
                }
            });

            self.gpip.gpadc_config.modify(|val| val.clear_fifo());
        }
    }

    /// Convert ADC raw value using calibration parameters
    #[inline]
    pub fn get_conv_value(&self, val: u32) -> u32 {
        let os1 = self.adc_os1;
        let os2 = self.adc_os2;
        let coe = self.adc_coe;

        if val < os1 {
            return 0;
        }

        let conv_val = if os2 < 0 {
            // Calculate thresholds
            let threshold1 = (os1 as f32 * 1.5) as u32;
            // For negative os2, (1.5*os1-os2) equals (1.5*os1+|os2|)
            let threshold2 = threshold1 + os2.unsigned_abs();

            if val < threshold1 {
                ((val - os1) as f32 / coe) as i32
            } else if val >= threshold2 {
                // For negative os2, (val-os2) equals (val+|os2|)
                ((val as i32 - os2) as f32 / coe) as i32
            } else {
                (val as f32 / coe) as i32
            }
        } else {
            if val < (os1 + os2 as u32) {
                ((val - os1) as f32 / coe) as i32
            } else {
                ((val - os2 as u32) as f32 / coe) as i32
            }
        };

        conv_val.max(0) as u32
    }

    /// Parse ADC raw results into meaningful data (no_std version)
    /// Returns the number of results parsed
    #[inline]
    pub fn adc_parse_result(
        &self,
        buffer: &[u32],
        results: &mut [AdcResult],
        hbn: &hbn::RegisterBlock,
    ) -> usize {
        if self.adc_config.is_none() {
            return 0;
        }

        let count = buffer.len().min(results.len());
        if count == 0 {
            return 0;
        }

        let resolution = hbn.gpadc_config_1.read().res_sel();
        let diff_mode = hbn.gpadc_config_2.read().is_diff_mode_enabled();
        let vref_sel = hbn.gpadc_config_2.read().is_vref_sel();

        let ref_mv = if vref_sel { 2000 } else { 3200 };

        let mut chan_vref = 0i32;

        if !diff_mode {
            if let Some(ref_chan) = self.adc_reference_channel {
                let ref_chan_idx = ref_chan as u8;

                for i in 0..count {
                    let raw_data = buffer[i];
                    let pos_chan_idx = (raw_data >> 21) as u8;

                    if pos_chan_idx == ref_chan_idx {
                        let conv_result = (raw_data & 0xFFFF) as u32;

                        let calibrated_value = self.get_conv_value(conv_result);

                        chan_vref = match resolution {
                            hbn::GpadcResolution::Bit12 => {
                                let val = (calibrated_value >> 4).min(4095);
                                (val as i32 * ref_mv) / 4096
                            }
                            hbn::GpadcResolution::Bit14 => {
                                let val = (calibrated_value >> 2).min(16383);
                                (val as i32 * ref_mv) / 16384
                            }
                            hbn::GpadcResolution::Bit16 => {
                                let val = calibrated_value.min(65535);
                                (val as i32 * ref_mv) / 65536
                            }
                        };
                        break;
                    }
                }
            }

            for i in 0..count {
                let raw_data = buffer[i];
                let pos_chan_idx = (raw_data >> 21) as u8;

                let pos_chan = Some(unsafe { core::mem::transmute(pos_chan_idx) });

                let conv_result = (raw_data & 0xFFFF) as u32;

                let calibrated_value = self.get_conv_value(conv_result);

                let (final_value, mut millivolt) = match resolution {
                    hbn::GpadcResolution::Bit12 => {
                        let val = (calibrated_value >> 4).min(4095);
                        let mv = (val as i32 * ref_mv) / 4096;
                        (val, mv)
                    }
                    hbn::GpadcResolution::Bit14 => {
                        let val = (calibrated_value >> 2).min(16383);
                        let mv = (val as i32 * ref_mv) / 16384;
                        (val, mv)
                    }
                    hbn::GpadcResolution::Bit16 => {
                        let val = calibrated_value.min(65535);
                        let mv = (val as i32 * ref_mv) / 65536;
                        (val, mv)
                    }
                };

                if let Some(ref_chan) = self.adc_reference_channel {
                    if pos_chan != Some(ref_chan) {
                        if chan_vref > 0 {
                            millivolt = millivolt * self.adc_reference_mv / chan_vref;
                        } else {
                            millivolt = 0;
                        }
                    }
                }

                results[i] = AdcResult {
                    pos_chan,
                    neg_chan: None,
                    value: final_value,
                    millivolt,
                };
            }
        } else {
            for i in 0..count {
                let raw_data = buffer[i];
                let pos_chan_idx = (raw_data >> 21) as u8;
                let neg_chan_idx = ((raw_data >> 16) & 0x1F) as u8;

                let pos_chan = unsafe { core::mem::transmute(pos_chan_idx) };
                let neg_chan = unsafe { core::mem::transmute(neg_chan_idx) };

                let mut tmp = raw_data;
                let mut is_negative = false;

                if tmp & 0x8000 != 0 {
                    tmp = !tmp;
                    tmp = tmp.wrapping_add(1);
                    is_negative = true;
                }

                let (final_value, millivolt) = match resolution {
                    hbn::GpadcResolution::Bit12 => {
                        let val = (((tmp & 0xFFFF) >> 4) as f32 / self.adc_coe) as u32;
                        let val = val.min(2047);
                        let mv = (val as i32 * ref_mv) / 2048;
                        (val, mv)
                    }
                    hbn::GpadcResolution::Bit14 => {
                        let val = (((tmp & 0xFFFF) >> 2) as f32 / self.adc_coe) as u32;
                        let val = val.min(8191);
                        let mv = (val as i32 * ref_mv) / 8192;
                        (val, mv)
                    }
                    hbn::GpadcResolution::Bit16 => {
                        let val = ((tmp & 0xFFFF) as f32 / self.adc_coe) as u32;
                        let val = val.min(32767);
                        let mv = (val as i32 * ref_mv) / 32768;
                        (val, mv)
                    }
                };

                let (signed_value, signed_millivolt) = if is_negative {
                    (-(final_value as i32), -millivolt)
                } else {
                    (final_value as i32, millivolt)
                };

                results[i] = AdcResult {
                    pos_chan,
                    neg_chan,
                    value: signed_value as u32,
                    millivolt: signed_millivolt,
                };
            }
        }

        count
    }

    /// Get internal temperature sensor's temperature.
    #[inline]
    pub fn adc_get_tsen_temp(&mut self, hbn: &hbn::RegisterBlock) -> f32 {
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
            ((v0 as f32 - v1 as f32) - self.tsen_offset as f32) / 7.753
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
