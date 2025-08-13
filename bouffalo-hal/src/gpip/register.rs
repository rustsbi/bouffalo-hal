//! Generic DAC, ADC and ACOMP interface control peripheral.

use core::ops::Deref;

use crate::{
    glb::{self, v2::AdcClockSource},
    hbn::{self, GpadcChannel, GpadcClkDivider, GpadcResolution, GpadcVref},
};

use volatile_register::RW;

/// Generic DAC, ADC and ACOMP interface control peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Gpadc configuration register.
    pub gpadc_config: RW<GpadcConfig>,
    /// Gpadc DMA read data register.
    pub gpadc_dma_rdata: RW<GpadcDmaRdata>,
    _reserved0: [u8; 24],
    /// Gpadc PIR training register.
    pub gpadc_pir_train: RW<GpadcPirTrain>,
    _reserved1: [u8; 28],
    /// Gpdac configuration register.
    pub gpdac_config: RW<GpdacConfig>,
    /// Gpdac DMA configuration register.
    pub gpdac_dma_config: RW<GpdacDmaConfig>,
    /// Gpdac DMA write data register.
    pub gpdac_dma_wdata: RW<GpdacDmaWdata>,
    /// Gpdac transmit FIFO status register.
    pub gpdac_tx_fifo_status: RW<GpdacTxFifoStatus>,
    _reserved2: [u8; 696],
    /// Gpdac control register.
    pub gpdac_ctrl: RW<GpdacCtrl>,
    /// Gpdac control a register.
    pub gpdac_actrl: RW<GpdacActrl>,
    /// Gpdac control b register.
    pub gpdac_bctrl: RW<GpdacBctrl>,
    /// Gpdac data register.
    pub gpdac_data: RW<GpdacData>,
}

/// Fifo threshold of gpadc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcFifoThreshold {
    OneData,
    FourData,
    EightData,
    SixteenData,
}

/// Generic Analog-to-Digital Converter configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConfig(u32);

impl GpadcConfig {
    const FIFO_THRES: u32 = 0x3 << 22;
    const FIFO_DAT_CNT: u32 = 0x3F << 16;
    const FIFO_RDY_MASK: u32 = 0x1 << 15;
    const FIFO_UNDERRUN_MASK: u32 = 0x1 << 14;
    const FIFO_OVERRUN_MASK: u32 = 0x1 << 13;
    const GPADC_RDY_MASK: u32 = 0x1 << 12;
    const FIFO_UNDERRUN_CLR: u32 = 0x1 << 10;
    const FIFO_OVERRUN_CLR: u32 = 0x1 << 9;
    const GPADC_RDY_CLR: u32 = 0x1 << 8;
    const FIFO_RDY: u32 = 0x1 << 7;
    const FIFO_UNDERRUN: u32 = 0x1 << 6;
    const FIFO_OVERRUN: u32 = 0x1 << 5;
    const GPADC_RDY: u32 = 0x1 << 4;
    const FIFO_FULL: u32 = 0x1 << 3;
    const FIFO_NE: u32 = 0x1 << 2;
    const FIFO_CLR: u32 = 0x1 << 1;
    const DMA_EN: u32 = 0x1;

    /// Set fifo threshold.
    #[inline]
    pub fn set_fifo_threshold(self, fifo_thres: GpadcFifoThreshold) -> Self {
        Self((self.0 & !Self::FIFO_THRES) | ((fifo_thres as u32) << 22))
    }
    /// Get fifo threshold.
    #[inline]
    pub fn fifo_threshold(self) -> GpadcFifoThreshold {
        match (self.0 & Self::FIFO_THRES) >> 22 {
            0 => GpadcFifoThreshold::OneData,
            1 => GpadcFifoThreshold::FourData,
            2 => GpadcFifoThreshold::EightData,
            3 => GpadcFifoThreshold::SixteenData,
            _ => unreachable!(),
        }
    }
    /// Get fifo data count.
    #[inline]
    pub fn fifo_data_count(self) -> u16 {
        ((self.0 & Self::FIFO_DAT_CNT) >> 16) as u16
    }
    /// Enable fifo ready interrupt.
    #[inline]
    pub fn enable_fifo_ready(self) -> Self {
        Self(self.0 & !Self::FIFO_RDY_MASK)
    }
    /// Disable fifo ready interrupt.
    #[inline]
    pub fn disable_fifo_ready(self) -> Self {
        Self(self.0 | Self::FIFO_RDY_MASK)
    }
    /// Check if fifo ready interrupt is enabled.
    #[inline]
    pub fn is_fifo_ready_enabled(self) -> bool {
        self.0 & Self::FIFO_RDY_MASK == 0
    }
    /// Enable fifo underrun interrupt.
    #[inline]
    pub fn enable_fifo_underrun(self) -> Self {
        Self(self.0 & !Self::FIFO_UNDERRUN_MASK)
    }
    /// Disable fifo underrun interrupt.
    #[inline]
    pub fn disable_fifo_underrun(self) -> Self {
        Self(self.0 | Self::FIFO_UNDERRUN_MASK)
    }
    /// Check if fifo underrun interrupt is enabled.
    #[inline]
    pub fn is_fifo_underrun_enabled(self) -> bool {
        self.0 & Self::FIFO_UNDERRUN_MASK == 0
    }
    /// Enable fifo overrun interrupt.
    #[inline]
    pub fn enable_fifo_overrun(self) -> Self {
        Self(self.0 & !Self::FIFO_OVERRUN_MASK)
    }
    /// Disable fifo overrun interrupt.
    #[inline]
    pub fn disable_fifo_overrun(self) -> Self {
        Self(self.0 | Self::FIFO_OVERRUN_MASK)
    }
    /// Check if fifo overrun interrupt is enabled.
    #[inline]
    pub fn is_fifo_overrun_enabled(self) -> bool {
        self.0 & Self::FIFO_OVERRUN_MASK == 0
    }
    /// Enable adc coversion ready interrupt.
    #[inline]
    pub fn enable_adc_ready(self) -> Self {
        Self(self.0 & !Self::GPADC_RDY_MASK)
    }
    /// Disable adc coversion ready interrupt.
    #[inline]
    pub fn disable_adc_ready(self) -> Self {
        Self(self.0 | Self::GPADC_RDY_MASK)
    }
    /// Check if adc coversion ready interrupt is enabled.
    #[inline]
    pub fn is_adc_ready_enabled(self) -> bool {
        self.0 & Self::GPADC_RDY_MASK == 0
    }
    /// Clear fifo underrun interrupt flag.
    #[inline]
    pub fn clear_fifo_underrun(self) -> Self {
        Self(self.0 | Self::FIFO_UNDERRUN_CLR)
    }
    /// Set fifo underrun interrupt flag bit.
    #[inline]
    pub fn set_fifo_underrun_clr_bit(self, bit: u8) -> Self {
        Self((self.0 & !Self::FIFO_UNDERRUN_CLR) | (Self::FIFO_UNDERRUN_CLR & ((bit as u32) << 10)))
    }
    /// Clear fifo overrun interrupt flag.
    #[inline]
    pub fn clear_fifo_overrun(self) -> Self {
        Self(self.0 | Self::FIFO_OVERRUN_CLR)
    }
    /// Set fifo overrun interrupt flag bit.
    #[inline]
    pub fn set_fifo_overrun_clr_bit(self, bit: u8) -> Self {
        Self((self.0 & !Self::FIFO_OVERRUN_CLR) | (Self::FIFO_OVERRUN_CLR & ((bit as u32) << 9)))
    }
    /// Clear adc coversion ready interrupt flag.
    #[inline]
    pub fn clear_adc_ready(self) -> Self {
        Self(self.0 | Self::GPADC_RDY_CLR)
    }
    /// Set adc coversion ready interrupt flag bit.
    #[inline]
    pub fn set_adc_ready_clr_bit(self, bit: u8) -> Self {
        Self((self.0 & !Self::GPADC_RDY_CLR) | (Self::GPADC_RDY_CLR & ((bit as u32) << 8)))
    }
    /// Check if fifo underrun interrupt occurs.
    #[inline]
    pub fn if_fifo_underrun_occurs(self) -> bool {
        self.0 & Self::FIFO_UNDERRUN != 0
    }
    /// Check if fifo overrun interrupt occurs.
    #[inline]
    pub fn if_fifo_overrun_occurs(self) -> bool {
        self.0 & Self::FIFO_OVERRUN != 0
    }
    /// Check if fifo coversion is ready.
    #[inline]
    pub fn is_fifo_ready(self) -> bool {
        self.0 & Self::FIFO_RDY != 0
    }
    /// Set fifo ready bit.
    #[inline]
    pub fn set_fifo_ready_bit(self, bit: u8) -> Self {
        Self((self.0 & !Self::FIFO_RDY) | (Self::FIFO_RDY & ((bit as u32) << 7)))
    }
    /// Check if adc coversion is ready.
    #[inline]
    pub fn is_adc_ready(self) -> bool {
        self.0 & Self::GPADC_RDY != 0
    }
    /// Check if fifo is full.
    #[inline]
    pub fn is_fifo_full(self) -> bool {
        self.0 & Self::FIFO_FULL != 0
    }
    /// Check if fifo is not empty.
    #[inline]
    pub fn is_fifo_not_empty(self) -> bool {
        self.0 & Self::FIFO_NE != 0
    }
    /// Clear fifo.
    #[inline]
    pub fn clear_fifo(self) -> Self {
        Self(self.0 | Self::FIFO_CLR)
    }
    /// Set fifo clear bit.
    #[inline]
    pub fn set_fifo_clear_bit(self, bit: u8) -> Self {
        Self((self.0 & !Self::FIFO_CLR) | (Self::FIFO_CLR & ((bit as u32) << 1)))
    }
    /// Enable dma.
    #[inline]
    pub fn enable_dma(self) -> Self {
        Self(self.0 | Self::DMA_EN)
    }
    /// Disable dma.
    #[inline]
    pub fn disable_dma(self) -> Self {
        Self(self.0 & !Self::DMA_EN)
    }
    /// Check if dma is enabled.
    #[inline]
    pub fn is_dma_enabled(self) -> bool {
        self.0 & Self::DMA_EN != 0
    }
}

/// Gpadc DMA read data register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcDmaRdata(u32);

impl GpadcDmaRdata {
    const DMA_RDATA: u32 = 0x03FF_FFFF;

    /// Get dma rdata.
    #[inline]
    pub fn dma_rdata(self) -> u32 {
        self.0 & Self::DMA_RDATA
    }
}

/// Gpadc PIR training register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcPirTrain(u32);

impl GpadcPirTrain {
    const PIR_STOP: u32 = 0x1 << 17;
    const PIR_TRAIN: u32 = 0x1 << 16;
    const PIR_CNT_V: u32 = 0x1F << 8;
    const PIR_EXTEND: u32 = 0x1F;

    /// Check if pir training stops.
    #[inline]
    pub fn if_pir_training_stops(self) -> bool {
        self.0 & Self::PIR_STOP != 0
    }
    /// Set pir training mode.
    #[inline]
    pub fn set_pir_training_mode(self, val: u8) -> Self {
        Self((self.0 & !Self::PIR_TRAIN) | (Self::PIR_TRAIN & ((val as u32) << 16)))
    }
    /// Get pir training mode.
    #[inline]
    pub fn pir_training_mode(self) -> u8 {
        ((self.0 & Self::PIR_TRAIN) >> 16) as u8
    }
    /// Get gpadc record extension counter value.
    #[inline]
    pub fn pir_counter_value(self) -> u8 {
        ((self.0 & Self::PIR_CNT_V) >> 8) as u8
    }
    /// Get gpadc record extension after pir interrupt.
    #[inline]
    pub fn pir_extend(self) -> u8 {
        ((self.0 & Self::PIR_EXTEND) >> 0) as u8
    }
    /// Set gpadc record extension after pir interrupt.
    #[inline]
    pub fn set_pir_extend(self, val: u8) -> Self {
        Self((self.0 & !Self::PIR_EXTEND) | (Self::PIR_EXTEND & ((val as u32) << 0)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacConfig(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacDmaConfig(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacDmaWdata(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacTxFifoStatus(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacCtrl(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacActrl(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacBctrl(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpdacData(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcTsenMode {
    InternalDiode = 0,
    ExternalDiode = 1,
}

/// Adc coniguration structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdcConfig {
    /// Clock divider for the adc.
    pub(crate) clk_div: GpadcClkDivider,
    /// Resolution of the adc.
    pub(crate) resolution: GpadcResolution,
    /// Voltage reference for the adc.
    pub(crate) vref: GpadcVref,
    /// Enable or disable the adc scan mode.
    pub(crate) scan_en: bool,
    /// Enable or disable the adc differential mode.
    pub(crate) diff_en: bool,
    /// Enable or disable the adc continuous conversion mode.
    pub(crate) continuous_conv_en: bool,
    /// Enable or disable the adc dma mode.
    pub(crate) dma_en: bool,
}

impl Default for AdcConfig {
    #[inline]
    fn default() -> Self {
        Self {
            clk_div: GpadcClkDivider::Div32,
            resolution: GpadcResolution::Bit16,
            vref: GpadcVref::V2p0,
            scan_en: false,
            diff_en: false,
            continuous_conv_en: true,
            dma_en: false,
        }
    }
}

impl AdcConfig {
    /// Set the clock divider for the adc.
    #[inline]
    pub fn set_clk_div(mut self, clk_div: GpadcClkDivider) -> Self {
        self.clk_div = clk_div;
        self
    }
    /// Set the resolution for the adc.
    #[inline]
    pub fn set_resolution(mut self, resolution: GpadcResolution) -> Self {
        self.resolution = resolution;
        self
    }
    /// Set the voltage reference for the adc.
    #[inline]
    pub fn set_vref(mut self, vref: GpadcVref) -> Self {
        self.vref = vref;
        self
    }
    /// Enable scan mode for the adc.
    #[inline]
    pub fn enable_scan(mut self) -> Self {
        self.scan_en = true;
        self
    }
    /// Disable scan mode for the adc.
    #[inline]
    pub fn disable_scan(mut self) -> Self {
        self.scan_en = false;
        self
    }
    /// Enable differential mode for the adc.
    #[inline]
    pub fn enable_diff_mode(mut self) -> Self {
        self.diff_en = true;
        self
    }
    /// Disable differential mode for the adc.
    #[inline]
    pub fn disable_diff_mode(mut self) -> Self {
        self.diff_en = false;
        self
    }
    /// Enable continuous covertion mode for the adc.
    #[inline]
    pub fn enable_continuous_conv(mut self) -> Self {
        self.continuous_conv_en = true;
        self
    }
    /// Disable continuous covertion mode for the adc.
    #[inline]
    pub fn disable_continuous_conv(mut self) -> Self {
        self.continuous_conv_en = false;
        self
    }
    /// Enable dma mode for the adc.
    #[inline]
    pub fn enable_dma(mut self) -> Self {
        self.dma_en = true;
        self
    }
    /// Disable dma mode for the adc.
    #[inline]
    pub fn disable_dma(mut self) -> Self {
        self.dma_en = false;
        self
    }
}

/// Adc command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AdcCommmand {
    ClearFifo,
    VbatEn,
}

/// Adc channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdcChannels {
    pub pos_ch: GpadcChannel,
    pub neg_ch: GpadcChannel,
}

/// Dac coniguration structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DacConfig {}

pub struct Gpip<G>
where
    G: Deref<Target = RegisterBlock>,
{
    pub(crate) gpip: G,
    pub(crate) adc_config: Option<AdcConfig>,
    pub(crate) adc_calibration_complete: bool,
    pub(crate) adc_coe: f32,
    pub(crate) adc_os1: u32,
    pub(crate) adc_os2: i32,
    pub(crate) tsen_offset: u32,
    pub(crate) dac_config: Option<DacConfig>,
    pub(crate) dac_calibration_complete: bool,
}

impl<G: Deref<Target = RegisterBlock>> Gpip<G> {
    /// Create a new Gpip instance with the given gpip peripheral and optional adc/dac configurations.
    #[inline]
    pub fn new(
        gpip: G,
        adc_config: Option<AdcConfig>,
        dac_config: Option<DacConfig>,
        glb: &glb::v2::RegisterBlock,
        hbn: &hbn::RegisterBlock,
    ) -> Self {
        unsafe {
            glb.clock_config_1.modify(|val| val.enable_gpip());
        }

        if adc_config.is_some() {
            let config = adc_config.unwrap();

            unsafe {
                glb.adc_config0.modify(|val| {
                    val.enable_div()
                        .set_divide(1)
                        .set_clk_src(AdcClockSource::Xclk)
                });
                // `enable_div` doesn't seem to work, temporarily use raw register write instead
                glb.adc_config0.modify(|val| val.write_val(0x181));

                for _ in 0..100 {
                    core::arch::asm!("nop");
                }

                hbn.gpadc_command.modify(|v| v.disable_global());
                hbn.gpadc_command
                    .modify(|v| v.enable_global().start_software_reset());

                for _ in 0..8 {
                    core::arch::asm!("nop");
                }

                hbn.gpadc_command.modify(|v| v.stop_software_reset());

                hbn.gpadc_config_1.modify(|v| {
                    let v = v
                        .set_v18_sel(2)
                        .set_v11_sel(1)
                        .set_clk_div_ratio(config.clk_div)
                        .set_res_sel(config.resolution);

                    #[cfg(feature = "bl702")]
                    let v = v.enable_lowv_det().enable_vcm_hyst_sel().enable_vcm_sel();

                    let v = if config.scan_en {
                        v.enable_scan().enable_clk_ana_inv()
                    } else {
                        v.disable_scan().disable_clk_ana_inv()
                    };
                    let v = if config.continuous_conv_en {
                        v.enable_continuous_conv()
                    } else {
                        v.disable_continuous_conv()
                    };
                    v
                });

                for _ in 0..8 {
                    core::arch::asm!("nop");
                }

                hbn.gpadc_config_2.modify(|v| {
                    let v = v
                        .set_dly_sel(2)
                        .enable_pga()
                        .set_pga1_gain(1)
                        .set_pga_os_cal(8)
                        .set_chop_mode(2) // Vref AZ and PGA chop on.
                        .set_pga_vcm(1) // PGA output common mode control 1.2V.
                        .set_vref_sel(matches!(config.vref, GpadcVref::V2p0));

                    #[cfg(feature = "bl702")]
                    let v = v.set_pga2_gain(0);
                    #[cfg(not(feature = "bl702"))]
                    let v = v.set_pga2_gain(1);

                    if config.diff_en {
                        v.enable_diff_mode()
                    } else {
                        v.disable_diff_mode()
                    }
                });

                hbn.gpadc_command.modify(|v| {
                    // Mic2 diff enable.
                    let v = v.enable_mic2_diff();
                    if config.diff_en {
                        v.unset_neg_gnd()
                    } else {
                        v.set_neg_gnd()
                    }
                });

                // Set calibration offset.
                hbn.gpadc_define.modify(|v| v.set_os_cal_data(0));

                gpip.gpadc_config.modify(|v| {
                    let v = v
                        .disable_fifo_overrun()
                        .disable_fifo_underrun()
                        .disable_adc_ready()
                        .clear_fifo_overrun()
                        .clear_fifo_underrun()
                        .clear_adc_ready()
                        .clear_fifo()
                        // Set this bit to 0.
                        .set_fifo_threshold(GpadcFifoThreshold::OneData)
                        .disable_dma();
                    #[cfg(feature = "bl702")]
                    let v = v.disable_fifo_ready().set_fifo_ready_bit(1);
                    v
                });
                gpip.gpadc_config.modify(|v| {
                    v.set_fifo_underrun_clr_bit(0)
                        .set_fifo_overrun_clr_bit(0)
                        .set_adc_ready_clr_bit(0)
                        .set_fifo_clear_bit(0)
                });
                hbn.gpadc_interrupt_state.modify(|v| {
                    v.disable_neg_satur_interrupt()
                        .disable_pos_satur_interrupt()
                });

                gpip.gpadc_config.modify(|val| {
                    if config.dma_en {
                        val.enable_dma()
                    } else {
                        val.disable_dma()
                    }
                });
            }
        }

        Self {
            gpip,
            adc_config,
            adc_calibration_complete: false,
            adc_coe: 0.0,
            adc_os1: 0,
            adc_os2: 0,
            tsen_offset: 0,
            dac_config,
            dac_calibration_complete: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GpadcConfig, GpadcDmaRdata, GpadcFifoThreshold, GpadcPirTrain, RegisterBlock};
    use core::mem::offset_of;

    #[test]
    fn struct_gpip_offset_functions() {
        assert_eq!(offset_of!(RegisterBlock, gpadc_config), 0x0);
        assert_eq!(offset_of!(RegisterBlock, gpadc_dma_rdata), 0x4);
        assert_eq!(offset_of!(RegisterBlock, gpadc_pir_train), 0x20);
        assert_eq!(offset_of!(RegisterBlock, gpdac_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, gpdac_dma_config), 0x44);
        assert_eq!(offset_of!(RegisterBlock, gpdac_dma_wdata), 0x48);
        assert_eq!(offset_of!(RegisterBlock, gpdac_tx_fifo_status), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, gpdac_ctrl), 0x308);
        assert_eq!(offset_of!(RegisterBlock, gpdac_actrl), 0x30c);
        assert_eq!(offset_of!(RegisterBlock, gpdac_bctrl), 0x310);
        assert_eq!(offset_of!(RegisterBlock, gpdac_data), 0x314);
    }

    #[test]
    fn struct_gpadc_config_functions() {
        let mut val = GpadcConfig(0);

        val = val.set_fifo_threshold(GpadcFifoThreshold::SixteenData);
        assert_eq!(val.fifo_threshold(), GpadcFifoThreshold::SixteenData);
        assert_eq!(val.0, 0x00C0_0000);

        val = val.set_fifo_threshold(GpadcFifoThreshold::EightData);
        assert_eq!(val.fifo_threshold(), GpadcFifoThreshold::EightData);
        assert_eq!(val.0, 0x0080_0000);

        val = val.set_fifo_threshold(GpadcFifoThreshold::FourData);
        assert_eq!(val.fifo_threshold(), GpadcFifoThreshold::FourData);
        assert_eq!(val.0, 0x0040_0000);

        val = val.set_fifo_threshold(GpadcFifoThreshold::OneData);
        assert_eq!(val.fifo_threshold(), GpadcFifoThreshold::OneData);
        assert_eq!(val.0, 0x0000_0000);

        val = GpadcConfig(0x003F_0000);
        assert_eq!(val.fifo_data_count(), 0x3F);

        val = GpadcConfig(0).enable_fifo_ready();
        assert!(val.is_fifo_ready_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_fifo_ready();
        assert!(!val.is_fifo_ready_enabled());
        assert_eq!(val.0, 0x0000_8000);

        val = GpadcConfig(0).enable_fifo_underrun();
        assert!(val.is_fifo_underrun_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_fifo_underrun();
        assert!(!val.is_fifo_underrun_enabled());
        assert_eq!(val.0, 0x0000_4000);

        val = GpadcConfig(0).enable_fifo_overrun();
        assert!(val.is_fifo_overrun_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_fifo_overrun();
        assert!(!val.is_fifo_overrun_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = GpadcConfig(0).enable_adc_ready();
        assert!(val.is_adc_ready_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_adc_ready();
        assert!(!val.is_adc_ready_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = GpadcConfig(0).clear_fifo_underrun();
        assert_eq!(val.0, 0x0000_0400);

        val = GpadcConfig(0).clear_fifo_overrun();
        assert_eq!(val.0, 0x0000_0200);

        val = GpadcConfig(0).clear_adc_ready();
        assert_eq!(val.0, 0x0000_0100);

        val = GpadcConfig(0).set_fifo_underrun_clr_bit(1);
        assert_eq!(val.0, 0x0000_0400);

        val = GpadcConfig(0).set_fifo_overrun_clr_bit(1);
        assert_eq!(val.0, 0x0000_0200);

        val = GpadcConfig(0).set_fifo_ready_bit(1);
        assert_eq!(val.0, 0x0000_0080);

        val = GpadcConfig(0).set_fifo_clear_bit(1);
        assert_eq!(val.0, 0x0000_0002);

        val = GpadcConfig(0x0000_0040);
        assert!(val.if_fifo_underrun_occurs());

        val = GpadcConfig(0x0000_0020);
        assert!(val.if_fifo_overrun_occurs());

        val = GpadcConfig(0x0000_0080);
        assert!(val.is_fifo_ready());

        val = GpadcConfig(0x0000_0010);
        assert!(val.is_adc_ready());

        val = GpadcConfig(0x0000_0008);
        assert!(val.is_fifo_full());

        val = GpadcConfig(0x0000_0004);
        assert!(val.is_fifo_not_empty());

        val = GpadcConfig(0).clear_fifo();
        assert_eq!(val.0, 0x0000_0002);

        val = GpadcConfig(0).enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x0000_0001);

        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_dma_rdata_functions() {
        let val = GpadcDmaRdata(0x03FF_FFFF);
        assert_eq!(val.dma_rdata(), 0x03FF_FFFF);

        let val = GpadcDmaRdata(0x0000_0000);
        assert_eq!(val.dma_rdata(), 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_pir_train_functions() {
        let mut val = GpadcPirTrain(0x0002_0000);
        assert!(val.if_pir_training_stops());

        val = GpadcPirTrain(0).set_pir_training_mode(1);
        assert_eq!(val.pir_training_mode(), 1);
        assert_eq!(val.0, 0x0001_0000);

        val = GpadcPirTrain(0x0000_1F00);
        assert_eq!(val.pir_counter_value(), 0x1F);

        val = GpadcPirTrain(0).set_pir_extend(0x1F);
        assert_eq!(val.pir_extend(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }
}
