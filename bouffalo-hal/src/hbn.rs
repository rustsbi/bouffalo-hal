//! Hibernation (deep-sleep) control peripheral.

use volatile_register::{RO, RW, WO};

/// Hibernation control registers.
#[repr(C)]
pub struct RegisterBlock {
    /// todo: fill in all registers
    /// Miscellaneous control register
    pub control: RW<u32>,
    /// Low bits of hibernate time
    pub time_lo: RW<u32>,
    /// High bits of hibernate time
    pub time_hi: RW<u32>,
    /// Low bits of Real-Time Clock time
    pub rtc_time_lo: RO<u32>,
    /// High bits of Real-Time Clock time
    pub rtc_time_hi: RO<u32>,
    /// Hibernate interrupt contol
    pub interrupt_mode: RW<u32>,
    /// Hibernate interrupt state
    pub interrupt_state: RO<u32>,
    /// Clear hibernate interrupt
    pub interrupt_clear: WO<u32>,
    /// Passive infrared sensor configuration
    pub pir_config: RW<PirConfig>,
    /// Passive infrared sensor voltage threshold
    pub pir_threshold: RW<u32>,
    /// Passive infrared sensor time interval
    pub pir_interval: RW<u32>,
    /// Brown-out reset function configuration
    pub bor_config: RW<u32>,
    /// Global hibernate configuration
    pub global: RW<Global>,
    /// Static Random-Access Memory hibernate control
    pub sram: RW<u32>,
    /// Always-on pad control register 0
    pub pad_control_0: RW<u32>,
    /// Always-on pad control register 1
    pub pad_control_1: RW<u32>,
    _reserved0: [u8; 448],
    /// 32-kHz internal RC oscillator control
    pub rc32k: RW<u32>,
    /// External crystal oscillator control
    pub xtal32k: RW<u32>,
    /// Real-Time Clock control and reset register 0
    pub rtc_control_0: RW<u32>,
    /// Real-Time Clock control and reset register 1
    pub rtc_control_1: RW<u32>,
    _reserved1: [u8; 0x6FC],
    pub gpadc_command: RW<GpadcCommand>,
    pub gpadc_config_1: RW<GpadcConfig1>,
    pub gpadc_config_2: RW<GpadcConfig2>,
    pub gpadc_converation_sequence_1: RW<GpadcConverationSequence1>,
    pub gpadc_converation_sequence_2: RW<GpadcConverationSequence2>,
    pub gpadc_converation_sequence_3: RW<GpadcConverationSequence3>,
    pub gpadc_converation_sequence_4: RW<GpadcConverationSequence4>,
    pub gpadc_status: RW<GpadcStatus>,
    pub gpadc_interrupt_state: RW<GpadcInterruptState>,
    pub gpadc_result: RW<GpadcResult>,
    pub gpadc_raw_result: RW<GpadcRawResult>,
    pub gpadc_define: RW<GpadcDefine>,
}

/// Configuration structure for hibernation global register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Global(u32);

impl Global {
    const ROOT_CLOCK_SOURCE_1: u32 = 1 << 0;
    const ROOT_CLOCK_SOURCE_2: u32 = 1 << 1;
    const UART_CLOCK_SOURCE_1: u32 = 1 << 2;
    const F32K_SELECT: u32 = 0x3 << 3;
    const ROOT_CLK_SEL: u32 = 0x1 << 6;
    const RESET_EVENT: u32 = 0x3f << 7;
    const CLEAR_RESET_EVENT: u32 = 1 << 13;
    const UART_CLOCK_SOURCE_2: u32 = 1 << 15;

    /// Set root clock source 1.
    #[inline]
    pub const fn set_root_clock_1(self, val: RootClockSource1) -> Self {
        Self((self.0 & !Self::ROOT_CLOCK_SOURCE_1) | (val as u32))
    }
    /// Get root clock source 1.
    #[inline]
    pub const fn root_clock_1(self) -> RootClockSource1 {
        match (self.0 & Self::ROOT_CLOCK_SOURCE_1) >> 0 {
            0 => RootClockSource1::RC32M,
            1 => RootClockSource1::Xtal,
            _ => unreachable!(),
        }
    }
    /// Set root clock source 2.
    #[inline]
    pub const fn set_root_clock_2(self, val: RootClockSource2) -> Self {
        Self((self.0 & !Self::ROOT_CLOCK_SOURCE_2) | ((val as u32) << 1))
    }
    /// Get root clock source 2.
    #[inline]
    pub const fn root_clock_2(self) -> RootClockSource2 {
        match (self.0 & Self::ROOT_CLOCK_SOURCE_2) >> 1 {
            0 => RootClockSource2::Xclk,
            1 => RootClockSource2::Pllsel,
            _ => unreachable!(),
        }
    }
    /// Set f32k source.
    #[inline]
    pub const fn set_f32k_source(self, val: F32kSource) -> Self {
        Self((self.0 & !Self::F32K_SELECT) | ((val as u32) << 3))
    }
    /// Get f32k source.
    #[inline]
    pub const fn f32k_source(self) -> F32kSource {
        match (self.0 & Self::F32K_SELECT) >> 3 {
            0 => F32kSource::RC32K,
            1 => F32kSource::Xtal32K,
            2 => F32kSource::Dig32K,
            _ => unreachable!(),
        }
    }
    /// Set root clock source.
    #[inline]
    pub const fn set_root_clock(self, val: RootClockSource) -> Self {
        Self((self.0 & !Self::ROOT_CLK_SEL) | (val as u32))
    }
    /// Get root clock source.
    #[inline]
    pub const fn root_clock(self) -> RootClockSource {
        match (self.0 & Self::ROOT_CLK_SEL) >> 6 {
            0 => RootClockSource::RC32M,
            1 => RootClockSource::Xtal,
            _ => unreachable!(),
        }
    }
    /// Clear reset event.
    #[inline]
    pub const fn clear_reset_event(self) -> Self {
        Self(self.0 | Self::CLEAR_RESET_EVENT)
    }
    /// Get reset event.
    #[inline]
    pub const fn reset_event(self) -> ResetEvent {
        match (self.0 & Self::RESET_EVENT) >> 7 {
            0 => ResetEvent::CpuM0,
            1 => ResetEvent::CpuLp,
            2 => ResetEvent::Bus1,
            3 => ResetEvent::Glb,
            4 => ResetEvent::Mix,
            5 => ResetEvent::Gpip,
            6 => ResetEvent::SecEng,
            7 => ResetEvent::TZ,
            8 => ResetEvent::Efuse,
            9 => ResetEvent::Dma,
            10 => ResetEvent::Psram,
            11 => ResetEvent::Usb,
            12 => ResetEvent::Emac,
            13 => ResetEvent::Audio,
            14 => ResetEvent::Dma2,
            15 => ResetEvent::Pds,
            16 => ResetEvent::Uart0,
            17 => ResetEvent::Uart1,
            18 => ResetEvent::Spi,
            19 => ResetEvent::I2c,
            20 => ResetEvent::Pwm,
            21 => ResetEvent::Timer,
            22 => ResetEvent::Irr,
            23 => ResetEvent::Uart2Can,
            24 => ResetEvent::I2s,
            25 => ResetEvent::Pdm,
            26 => ResetEvent::Wifi,
            27 => ResetEvent::Ble,
            28 => ResetEvent::CpuD0,
            29 => ResetEvent::Bus2,
            30 => ResetEvent::MmMisc,
            31 => ResetEvent::MmDma,
            32 => ResetEvent::Mm2ddma,
            33 => ResetEvent::MmUart,
            34 => ResetEvent::MmI2c,
            35 => ResetEvent::MmIpc,
            36 => ResetEvent::MmTimer,
            37 => ResetEvent::UhsCtrl,
            38 => ResetEvent::DispTsrc,
            39 => ResetEvent::Nr3dCtrl,
            40 => ResetEvent::Dvp2busA,
            41 => ResetEvent::Dvp2busB,
            42 => ResetEvent::Dvp2busC,
            43 => ResetEvent::Dvp2busD,
            44 => ResetEvent::Dvp2busE,
            45 => ResetEvent::Dvp2busF,
            46 => ResetEvent::Dvp2busG,
            47 => ResetEvent::Dvp2busH,
            48 => ResetEvent::Jdec,
            49 => ResetEvent::Blai,
            _ => unreachable!(),
        }
    }
    /// Set uart clock source.
    #[inline]
    pub const fn set_uart_clock_source(self, val: UartClockSource) -> Self {
        Self(
            (self.0 & !((Self::UART_CLOCK_SOURCE_1 << 13) | Self::UART_CLOCK_SOURCE_2))
                | ((val as u32) << 15),
        )
    }
    /// Get uart clock source.
    #[inline]
    pub const fn uart_clock_source(self) -> UartClockSource {
        match (self.0 & ((Self::UART_CLOCK_SOURCE_1 << 13) | Self::UART_CLOCK_SOURCE_2)) >> 15 {
            0 => UartClockSource::McuBclk,
            1 => UartClockSource::MuxPll160M,
            2 => UartClockSource::Xclk,
            _ => unreachable!(),
        }
    }
}

/// Root clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootClockSource {
    /// Internal 32-MHz RC oscillator
    RC32M = 0,
    /// External crystal oscillator
    Xtal = 1,
}

/// Root clock source 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootClockSource1 {
    /// Internal 32-MHz RC oscillator
    RC32M = 0,
    /// External crystal oscillator
    Xtal = 1,
}

/// Root clock source 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootClockSource2 {
    /// External clock
    Xclk = 0,
    /// PLL select
    Pllsel = 1,
}

/// F32k clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F32kSource {
    /// Internal 32-kHz RC oscillator
    RC32K = 0,
    /// External 32-kHz crystal oscillator
    Xtal32K = 1,
    /// Digital 32-kHz clock
    Dig32K = 2,
}

/// Uart clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartClockSource {
    /// Microcontroller bus clock
    McuBclk = 0,
    /// 160-MHz mutiplexer PLL
    MuxPll160M = 1,
    /// External clock
    Xclk = 2,
}

/// Reset event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetEvent {
    CpuM0 = 0,
    CpuLp = 1,
    Bus1 = 2,
    Glb = 3,
    Mix = 4,
    Gpip = 5,
    SecEng = 6,
    TZ = 7,
    Efuse = 8,
    Dma = 9,
    Psram = 10,
    Usb = 11,
    Emac = 12,
    Audio = 13,
    Dma2 = 14,
    Pds = 15,
    Uart0 = 16,
    Uart1 = 17,
    Spi = 18,
    I2c = 19,
    Pwm = 20,
    Timer = 21,
    Irr = 22,
    Uart2Can = 23,
    I2s = 24,
    Pdm = 25,
    Wifi = 26,
    Ble = 27,
    CpuD0 = 28,
    Bus2 = 29,
    MmMisc = 30,
    MmDma = 31,
    Mm2ddma = 32,
    MmUart = 33,
    MmI2c = 34,
    MmIpc = 35,
    MmTimer = 36,
    UhsCtrl = 37,
    DispTsrc = 38,
    Nr3dCtrl = 39,
    Dvp2busA = 40,
    Dvp2busB = 41,
    Dvp2busC = 42,
    Dvp2busD = 43,
    Dvp2busE = 44,
    Dvp2busF = 45,
    Dvp2busG = 46,
    Dvp2busH = 47,
    Jdec = 48,
    Blai = 49,
}

/// Passive infrared sensor configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PirConfig(u32);

impl PirConfig {
    // const GPADC_CS: u32 = 0x1 << 8;
    const PIR_EN: u32 = 0x1 << 7;
    // const PIR_DIS: u32 = 0x3 << 4;
    // const PIR_LPF_SEL: u32 = 0x1 << 2;
    // const PIR_HPF_SEL: u32 = 0x3;

    /// Enable passive infrared sensor.
    #[inline]
    pub const fn enable_pir(self) -> Self {
        Self(self.0 | Self::PIR_EN)
    }
    /// Disable passive infrared sensor.
    #[inline]
    pub const fn disable_pir(self) -> Self {
        Self(self.0 & !Self::PIR_EN)
    }
    /// Check if passive infrared sensor is enabled.
    #[inline]
    pub const fn is_pir_enabled(self) -> bool {
        self.0 & Self::PIR_EN != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcCommand(u32);

impl GpadcCommand {
    const SEN_TEST_EN_V2: u32 = 0x1 << 31;
    const SEN_TEST_EN_V1: u32 = 0x1 << 30;
    const SEN_SEL_MASK_V1: u32 = 0x3 << 28;
    const SEN_SEL_MASK_V2: u32 = 0x7 << 28;
    const CHIP_SEN_PU: u32 = 0x1 << 27;
    const MICBOOST_32DB_EN: u32 = 0x1 << 23;
    const MIC_PGA2_GAIN_MASK: u32 = 0x3 << 21;
    const MIC1_DIFF: u32 = 0x1 << 20;
    const MIC2_DIFF: u32 = 0x1 << 19;
    const DWA_EN: u32 = 0x1 << 18;
    const RCAL_EN: u32 = 0x1 << 17;
    const BYP_MICBOOST: u32 = 0x1 << 16;
    const MICPGA_EN: u32 = 0x1 << 15;
    const MICBIAS_EN: u32 = 0x1 << 14;
    const NEG_GND: u32 = 0x1 << 13;
    const POS_SEL: u32 = 0x1F << 8;
    const NEG_SEL: u32 = 0x1F << 3;
    const SOFT_RST: u32 = 0x1 << 2;
    const CONV_START: u32 = 0x1 << 1;
    const GLOBAL_EN: u32 = 0x1;

    /// Enable sensor test mode for version 2.
    #[inline]
    pub const fn enable_sensor_test_v2(self) -> Self {
        Self(self.0 | Self::SEN_TEST_EN_V2)
    }
    /// Disable sensor test mode for version 2.
    #[inline]
    pub const fn disable_sensor_test_v2(self) -> Self {
        Self(self.0 & !Self::SEN_TEST_EN_V2)
    }
    /// Check if sensor test mode for version 2 is enabled.
    #[inline]
    pub const fn is_sensor_test_v2_enabled(self) -> bool {
        self.0 & Self::SEN_TEST_EN_V2 != 0
    }
    /// Enable sensor test mode for version 1.
    #[inline]
    pub const fn enable_sensor_test_v1(self) -> Self {
        Self(self.0 | Self::SEN_TEST_EN_V1)
    }
    /// Disable sensor test mode for version 1.
    #[inline]
    pub const fn disable_sensor_test_v1(self) -> Self {
        Self(self.0 & !Self::SEN_TEST_EN_V1)
    }
    /// Check if sensor test mode for version 1 is enabled.
    #[inline]
    pub const fn is_sensor_test_v1_enabled(self) -> bool {
        self.0 & Self::SEN_TEST_EN_V1 != 0
    }
    /// Set sensor for version 1.
    #[inline]
    pub const fn set_sensor_v1(self, sensor: u8) -> Self {
        Self((self.0 & !Self::SEN_SEL_MASK_V1) | (Self::SEN_SEL_MASK_V1 & ((sensor as u32) << 28)))
    }
    /// Get sensor for version 1.
    #[inline]
    pub const fn sensor_v1(self) -> u8 {
        ((self.0 & Self::SEN_SEL_MASK_V1) >> 28) as u8
    }
    /// Set sensor for version 2.
    #[inline]
    pub const fn set_sensor_v2(self, sensor: u8) -> Self {
        Self((self.0 & !Self::SEN_SEL_MASK_V2) | (Self::SEN_SEL_MASK_V2 & ((sensor as u32) << 28)))
    }
    /// Get sensor for version 2.
    #[inline]
    pub const fn sensor_v2(self) -> u8 {
        ((self.0 & Self::SEN_SEL_MASK_V2) >> 28) as u8
    }
    /// Enable on-chip temperature sensor pull-up.
    #[inline]
    pub const fn enable_chip_sen_pu(self) -> Self {
        Self(self.0 | Self::CHIP_SEN_PU)
    }
    /// Disable on-chip temperature sensor pull-up.
    #[inline]
    pub const fn disable_chip_sen_pu(self) -> Self {
        Self(self.0 & !Self::CHIP_SEN_PU)
    }
    /// Check if on-chip temperature sensor pull-up is enabled.
    #[inline]
    pub const fn is_chip_sen_pu_enabled(self) -> bool {
        self.0 & Self::CHIP_SEN_PU != 0
    }
    /// Enable microphone boost at 32dB.
    #[inline]
    pub const fn enable_micboost_32db(self) -> Self {
        Self(self.0 | Self::MICBOOST_32DB_EN)
    }
    /// Disable microphone boost at 32dB.
    #[inline]
    pub const fn disable_micboost_32db(self) -> Self {
        Self(self.0 & !Self::MICBOOST_32DB_EN)
    }
    /// Check if microphone boost at 32dB is enabled.
    #[inline]
    pub const fn is_micboost_32db_enabled(self) -> bool {
        self.0 & Self::MICBOOST_32DB_EN != 0
    }
    /// Set microphone programmable gain amplifier 2 gain.
    #[inline]
    pub const fn set_mic_pga2_gain(self, gain: u8) -> Self {
        Self(
            (self.0 & !Self::MIC_PGA2_GAIN_MASK)
                | (Self::MIC_PGA2_GAIN_MASK & ((gain as u32) << 21)),
        )
    }
    /// Get microphone programmable gain amplifier 2 gain.
    #[inline]
    pub const fn mic_pga2_gain(self) -> u8 {
        ((self.0 & Self::MIC_PGA2_GAIN_MASK) >> 21) as u8
    }
    /// Enable differential mode for microphone 1.
    #[inline]
    pub const fn enable_mic1_diff(self) -> Self {
        Self(self.0 | Self::MIC1_DIFF)
    }
    /// Disable differential mode for microphone 1.
    #[inline]
    pub const fn disable_mic1_diff(self) -> Self {
        Self(self.0 & !Self::MIC1_DIFF)
    }
    /// Check if differential mode for microphone 1 is enabled.
    #[inline]
    pub const fn is_mic1_diff_enabled(self) -> bool {
        self.0 & Self::MIC1_DIFF != 0
    }
    /// Enable differential mode for microphone 2.
    #[inline]
    pub const fn enable_mic2_diff(self) -> Self {
        Self(self.0 | Self::MIC2_DIFF)
    }
    /// Disable differential mode for microphone 2.
    #[inline]
    pub const fn disable_mic2_diff(self) -> Self {
        Self(self.0 & !Self::MIC2_DIFF)
    }
    /// Check if differential mode for microphone 2 is enabled.
    #[inline]
    pub const fn is_mic2_diff_enabled(self) -> bool {
        self.0 & Self::MIC2_DIFF != 0
    }
    /// Enable dynamic element weighting.
    #[inline]
    pub const fn enable_dwa(self) -> Self {
        Self(self.0 | Self::DWA_EN)
    }
    /// Disable dynamic element weighting.
    #[inline]
    pub const fn disable_dwa(self) -> Self {
        Self(self.0 & !Self::DWA_EN)
    }
    /// Check if dynamic element weighting is enabled.
    #[inline]
    pub const fn is_dwa_enabled(self) -> bool {
        self.0 & Self::DWA_EN != 0
    }
    /// Enable internal resistor calibration.
    #[inline]
    pub const fn enable_rcal(self) -> Self {
        Self(self.0 | Self::RCAL_EN)
    }
    /// Disable internal resistor calibration.
    #[inline]
    pub const fn disable_rcal(self) -> Self {
        Self(self.0 & !Self::RCAL_EN)
    }
    /// Check if internal resistor calibration is enabled.
    #[inline]
    pub const fn is_rcal_enabled(self) -> bool {
        self.0 & Self::RCAL_EN != 0
    }
    /// Enable bypass of microphone boost.
    #[inline]
    pub const fn enable_byp_micboost(self) -> Self {
        Self(self.0 | Self::BYP_MICBOOST)
    }
    /// Disable bypass of microphone boost.
    #[inline]
    pub const fn disable_byp_micboost(self) -> Self {
        Self(self.0 & !Self::BYP_MICBOOST)
    }
    /// Check if bypass of microphone boost is enabled.
    #[inline]
    pub const fn is_byp_micboost_enabled(self) -> bool {
        self.0 & Self::BYP_MICBOOST != 0
    }
    /// Enable microphone programmable gain amplifier.
    #[inline]
    pub const fn enable_micpga(self) -> Self {
        Self(self.0 | Self::MICPGA_EN)
    }
    /// Disable microphone programmable gain amplifier.
    #[inline]
    pub const fn disable_micpga(self) -> Self {
        Self(self.0 & !Self::MICPGA_EN)
    }
    /// Check if microphone programmable gain amplifier is enabled.
    #[inline]
    pub const fn is_micpga_enabled(self) -> bool {
        self.0 & Self::MICPGA_EN != 0
    }
    /// Enable microphone bias voltage.
    #[inline]
    pub const fn enable_micbias(self) -> Self {
        Self(self.0 | Self::MICBIAS_EN)
    }
    /// Disable microphone bias voltage.
    #[inline]
    pub const fn disable_micbias(self) -> Self {
        Self(self.0 & !Self::MICBIAS_EN)
    }
    /// Check if microphone bias voltage is enabled.
    #[inline]
    pub const fn is_micbias_enabled(self) -> bool {
        self.0 & Self::MICBIAS_EN != 0
    }
    /// Set the negative input to ground.
    #[inline]
    pub const fn set_neg_gnd(self) -> Self {
        Self(self.0 | Self::NEG_GND)
    }
    /// Unset the negative input to ground.
    #[inline]
    pub const fn unset_neg_gnd(self) -> Self {
        Self(self.0 & !Self::NEG_GND)
    }
    /// Check if the negative input is set to ground.
    #[inline]
    pub const fn is_neg_gnd_set(self) -> bool {
        self.0 & Self::NEG_GND != 0
    }
    /// Set the positive input selection.
    #[inline]
    pub const fn set_pos_sel(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::POS_SEL) | (Self::POS_SEL & ((channel as u32) << 8)))
    }
    /// Get the positive input selection.
    #[inline]
    pub const fn pos_sel(self) -> GpadcChannel {
        match ((self.0 & Self::POS_SEL) >> 8) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set the negative input selection.
    #[inline]
    pub const fn set_neg_sel(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::NEG_SEL) | (Self::NEG_SEL & ((channel as u32) << 3)))
    }
    /// Get the negative input selection.
    #[inline]
    pub const fn neg_sel(self) -> GpadcChannel {
        match ((self.0 & Self::NEG_SEL) >> 3) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Start software reset of the adc.
    #[inline]
    pub const fn start_software_reset(self) -> Self {
        Self(self.0 | Self::SOFT_RST)
    }
    /// Stop software reset of the adc.
    #[inline]
    pub const fn stop_software_reset(self) -> Self {
        Self(self.0 & !Self::SOFT_RST)
    }
    /// Start adc conversion.
    #[inline]
    pub const fn start_conversion(self) -> Self {
        Self(self.0 | Self::CONV_START)
    }
    /// Stop adc conversion.
    #[inline]
    pub const fn stop_conversion(self) -> Self {
        Self(self.0 & !Self::CONV_START)
    }
    /// Enable the adc.
    #[inline]
    pub const fn enable_global(self) -> Self {
        Self(self.0 | Self::GLOBAL_EN)
    }
    /// Disable the adc.
    #[inline]
    pub const fn disable_global(self) -> Self {
        Self(self.0 & !Self::GLOBAL_EN)
    }
    /// Check if the Analog-to-Digital Converter is enabled.
    #[inline]
    pub const fn is_global_enabled(self) -> bool {
        self.0 & Self::GLOBAL_EN != 0
    }
}

/// Gpadc clock divider selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcClkDivider {
    Div4 = 1,
    Div8 = 2,
    Div12 = 3,
    Div16 = 4,
    Div20 = 5,
    Div24 = 6,
    Div32 = 7,
}

/// Gpadc resolution selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcResolution {
    Bit12 = 0,
    Bit14 = 2,
    Bit16 = 4,
}

/// Gpadc voltage reference selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcVref {
    V3p2 = 0,
    V2p0 = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConfig1(u32);

impl GpadcConfig1 {
    const V18_SEL_MASK: u32 = 0x3 << 29;
    const V11_SEL_MASK: u32 = 0x3 << 27;
    const DITHER_EN: u32 = 0x1 << 26;
    const SCAN_EN: u32 = 0x1 << 25;
    const SCAN_LENGTH_MASK: u32 = 0xF << 21;
    const CLK_DIV_RATIO_MASK: u32 = 0x7 << 18;
    const CLK_ANA_INV: u32 = 0x1 << 17;
    const CLK_ANA_DLY_EN: u32 = 0x1 << 16;
    const CLK_ANA_DLY_MASK: u32 = 0xF << 12;
    const PWM_TRG_EN: u32 = 0x1 << 11;
    const LOWV_DET_EN: u32 = 0x1 << 10;
    const VCM_HYST_SEL: u32 = 0x1 << 9;
    const VCM_SEL_EN: u32 = 0x1 << 8;
    const RES_SEL_MASK: u32 = 0x7 << 2;
    const CONT_CONV_EN: u32 = 0x1 << 1;
    const CAL_OS_EN: u32 = 0x1 << 0;

    /// Set internal voltage regulator 1.8V selection.
    #[inline]
    pub fn set_v18_sel(self, sel: u8) -> Self {
        Self((self.0 & !Self::V18_SEL_MASK) | (Self::V18_SEL_MASK & ((sel as u32) << 29)))
    }
    /// Get internal voltage regulator 1.8V selection.
    #[inline]
    pub fn v18_sel(self) -> u8 {
        ((self.0 & Self::V18_SEL_MASK) >> 29) as u8
    }
    /// Set internal voltage regulator 1.1V selection.
    #[inline]
    pub fn set_v11_sel(self, sel: u8) -> Self {
        Self((self.0 & !Self::V11_SEL_MASK) | (Self::V11_SEL_MASK & ((sel as u32) << 27)))
    }
    /// Get internal voltage regulator 1.1V selection.
    #[inline]
    pub fn v11_sel(self) -> u8 {
        ((self.0 & Self::V11_SEL_MASK) >> 27) as u8
    }
    /// Enable dither.
    #[inline]
    pub fn enable_dither(self) -> Self {
        Self(self.0 | Self::DITHER_EN)
    }
    /// Disable dither.
    #[inline]
    pub fn disable_dither(self) -> Self {
        Self(self.0 & !Self::DITHER_EN)
    }
    /// Check if dither is enabled.
    #[inline]
    pub fn is_dither_enabled(self) -> bool {
        self.0 & Self::DITHER_EN != 0
    }
    /// Enable scan mode.
    #[inline]
    pub fn enable_scan(self) -> Self {
        Self(self.0 | Self::SCAN_EN)
    }
    /// Disable scan mode.
    #[inline]
    pub fn disable_scan(self) -> Self {
        Self(self.0 & !Self::SCAN_EN)
    }
    /// Check if scan mode is enabled.
    #[inline]
    pub fn is_scan_enabled(self) -> bool {
        self.0 & Self::SCAN_EN != 0
    }
    /// Set scan length.
    #[inline]
    pub fn set_scan_length(self, length: u8) -> Self {
        Self(
            (self.0 & !Self::SCAN_LENGTH_MASK) | (Self::SCAN_LENGTH_MASK & ((length as u32) << 21)),
        )
    }
    /// Get scan length.
    #[inline]
    pub fn scan_length(self) -> u8 {
        ((self.0 & Self::SCAN_LENGTH_MASK) >> 21) as u8
    }
    /// Set clock division ratio.
    #[inline]
    pub fn set_clk_div_ratio(self, ratio: GpadcClkDivider) -> Self {
        Self(
            (self.0 & !Self::CLK_DIV_RATIO_MASK)
                | (Self::CLK_DIV_RATIO_MASK & ((ratio as u32) << 18)),
        )
    }
    /// Get clock division ratio.
    #[inline]
    pub fn clk_div_ratio(self) -> GpadcClkDivider {
        match ((self.0 & Self::CLK_DIV_RATIO_MASK) >> 18) as u8 {
            1 => GpadcClkDivider::Div4,
            2 => GpadcClkDivider::Div8,
            3 => GpadcClkDivider::Div12,
            4 => GpadcClkDivider::Div16,
            5 => GpadcClkDivider::Div20,
            6 => GpadcClkDivider::Div24,
            7 => GpadcClkDivider::Div32,
            _ => unreachable!(),
        }
    }
    /// Enable invert analog clock.
    #[inline]
    pub fn enable_clk_ana_inv(self) -> Self {
        Self(self.0 | Self::CLK_ANA_INV)
    }
    /// Disable invert analog clock.
    #[inline]
    pub fn disable_clk_ana_inv(self) -> Self {
        Self(self.0 & !Self::CLK_ANA_INV)
    }
    /// Check if analog clock is inverted.
    #[inline]
    pub fn is_clk_ana_inv_enabled(self) -> bool {
        self.0 & Self::CLK_ANA_INV != 0
    }
    /// Enable analog clock delay.
    #[inline]
    pub fn enable_clk_ana_dly(self) -> Self {
        Self(self.0 | Self::CLK_ANA_DLY_EN)
    }
    /// Disable analog clock delay.
    #[inline]
    pub fn disable_clk_ana_dly(self) -> Self {
        Self(self.0 & !Self::CLK_ANA_DLY_EN)
    }
    /// Check if analog clock delay is enabled.
    #[inline]
    pub fn is_clk_ana_dly_enabled(self) -> bool {
        self.0 & Self::CLK_ANA_DLY_EN != 0
    }
    /// Set analog clock delay.
    #[inline]
    pub fn set_clk_ana_dly(self, delay: u8) -> Self {
        Self((self.0 & !Self::CLK_ANA_DLY_MASK) | (Self::CLK_ANA_DLY_MASK & ((delay as u32) << 12)))
    }
    /// Get analog clock delay.
    #[inline]
    pub fn clk_ana_dly(self) -> u8 {
        ((self.0 & Self::CLK_ANA_DLY_MASK) >> 12) as u8
    }
    /// Enable pwm trigger.
    #[inline]
    pub fn enable_pwm_trigger(self) -> Self {
        Self(self.0 | Self::PWM_TRG_EN)
    }
    /// Disable pwm trigger.
    #[inline]
    pub fn disable_pwm_trigger(self) -> Self {
        Self(self.0 & !Self::PWM_TRG_EN)
    }
    /// Check if pwm trigger is enabled.
    #[inline]
    pub fn is_pwm_trigger_enabled(self) -> bool {
        self.0 & Self::PWM_TRG_EN != 0
    }
    /// Enable low voltage detection.
    #[inline]
    pub fn enable_lowv_det(self) -> Self {
        Self(self.0 | Self::LOWV_DET_EN)
    }
    /// Disable low voltage detection.
    #[inline]
    pub fn disable_lowv_det(self) -> Self {
        Self(self.0 & !Self::LOWV_DET_EN)
    }
    /// Check if low voltage detection is enabled.
    #[inline]
    pub fn is_lowv_det_enabled(self) -> bool {
        self.0 & Self::LOWV_DET_EN != 0
    }
    /// Enable vcm hysteresis selection.
    #[inline]
    pub fn enable_vcm_hyst_sel(self) -> Self {
        Self(self.0 | Self::VCM_HYST_SEL)
    }
    /// Disable vcm hysteresis selection.
    #[inline]
    pub fn disable_vcm_hyst_sel(self) -> Self {
        Self(self.0 & !Self::VCM_HYST_SEL)
    }
    /// Check if vcm hysteresis selection is enabled.
    #[inline]
    pub fn is_vcm_hyst_sel_enabled(self) -> bool {
        self.0 & Self::VCM_HYST_SEL != 0
    }
    /// Enable vcm selection.
    #[inline]
    pub fn enable_vcm_sel(self) -> Self {
        Self(self.0 | Self::VCM_SEL_EN)
    }
    /// Disable vcm selection.
    #[inline]
    pub fn disable_vcm_sel(self) -> Self {
        Self(self.0 & !Self::VCM_SEL_EN)
    }
    /// Check if vcm selection is enabled.
    #[inline]
    pub fn is_vcm_sel_enabled(self) -> bool {
        self.0 & Self::VCM_SEL_EN != 0
    }
    /// Set resolution selection.
    #[inline]
    pub fn set_res_sel(self, res: GpadcResolution) -> Self {
        Self((self.0 & !Self::RES_SEL_MASK) | (Self::RES_SEL_MASK & ((res as u32) << 2)))
    }
    /// Get resolution selection.
    #[inline]
    pub fn res_sel(self) -> GpadcResolution {
        match ((self.0 & Self::RES_SEL_MASK) >> 2) as u8 {
            0 => GpadcResolution::Bit12,
            2 => GpadcResolution::Bit14,
            4 => GpadcResolution::Bit16,
            _ => unreachable!(),
        }
    }
    /// Enable continuous conversion.
    #[inline]
    pub fn enable_continuous_conv(self) -> Self {
        Self(self.0 | Self::CONT_CONV_EN)
    }
    /// Disable continuous conversion.
    #[inline]
    pub fn disable_continuous_conv(self) -> Self {
        Self(self.0 & !Self::CONT_CONV_EN)
    }
    /// Check if continuous conversion is enabled.
    #[inline]
    pub fn is_continuous_conv_enabled(self) -> bool {
        self.0 & Self::CONT_CONV_EN != 0
    }
    /// Enable calibration offset.
    #[inline]
    pub fn enable_cal_os(self) -> Self {
        Self(self.0 | Self::CAL_OS_EN)
    }
    /// Disable calibration offset.
    #[inline]
    pub fn disable_cal_os(self) -> Self {
        Self(self.0 & !Self::CAL_OS_EN)
    }
    /// Check if calibration offset is enabled.
    #[inline]
    pub fn is_cal_os_enabled(self) -> bool {
        self.0 & Self::CAL_OS_EN != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConfig2(u32);

impl GpadcConfig2 {
    const TSVBE_LOW: u32 = 0x1 << 31;
    const DLY_SEL_MASK: u32 = 0x7 << 28;
    const PGA1_GAIN_MASK: u32 = 0x7 << 25;
    const PGA2_GAIN_MASK: u32 = 0x7 << 22;
    const TEST_SEL_MASK: u32 = 0x7 << 19;
    const TEST_EN: u32 = 0x1 << 18;
    const BIAS_SEL: u32 = 0x1 << 17;
    const CHOP_MODE_MASK: u32 = 0x3 << 15;
    const PGA_VCMI_EN: u32 = 0x1 << 14;
    const PGA_EN: u32 = 0x1 << 13;
    const PGA_OS_CAL_MASK: u32 = 0xf << 9;
    const PGA_VCM_MASK: u32 = 0x3 << 7;
    const TS_EN: u32 = 0x1 << 6;
    const TSEXT_SEL: u32 = 0x1 << 5;
    const VBAT_EN: u32 = 0x1 << 4;
    const VREF_SEL: u32 = 0x1 << 3;
    const DIFF_MODE: u32 = 0x1 << 2;

    /// Enable temperature sensor voltage reference low mode.
    #[inline]
    pub fn enable_tsvbe_low(self) -> Self {
        Self(self.0 | Self::TSVBE_LOW)
    }
    /// Disable temperature sensor voltage reference low mode.
    #[inline]
    pub fn disable_tsvbe_low(self) -> Self {
        Self(self.0 & !Self::TSVBE_LOW)
    }
    /// Check if temperature sensor voltage reference low mode is enabled.
    #[inline]
    pub fn is_tsvbe_low_enabled(self) -> bool {
        self.0 & Self::TSVBE_LOW != 0
    }
    /// Set delay selection.
    #[inline]
    pub fn set_dly_sel(self, sel: u8) -> Self {
        Self((self.0 & !Self::DLY_SEL_MASK) | (Self::DLY_SEL_MASK & ((sel as u32) << 28)))
    }
    /// Get delay selection.
    #[inline]
    pub fn dly_sel(self) -> u8 {
        ((self.0 & Self::DLY_SEL_MASK) >> 28) as u8
    }
    /// Set PGA1 gain.
    #[inline]
    pub fn set_pga1_gain(self, gain: u8) -> Self {
        Self((self.0 & !Self::PGA1_GAIN_MASK) | (Self::PGA1_GAIN_MASK & ((gain as u32) << 25)))
    }
    /// Get PGA1 gain.
    #[inline]
    pub fn pga1_gain(self) -> u8 {
        ((self.0 & Self::PGA1_GAIN_MASK) >> 25) as u8
    }
    /// Set PGA2 gain.
    #[inline]
    pub fn set_pga2_gain(self, gain: u8) -> Self {
        Self((self.0 & !Self::PGA2_GAIN_MASK) | (Self::PGA2_GAIN_MASK & ((gain as u32) << 22)))
    }
    /// Get PGA2 gain.
    #[inline]
    pub fn pga2_gain(self) -> u8 {
        ((self.0 & Self::PGA2_GAIN_MASK) >> 22) as u8
    }
    /// Set test selection.
    #[inline]
    pub fn set_test_sel(self, sel: u8) -> Self {
        Self((self.0 & !Self::TEST_SEL_MASK) | (Self::TEST_SEL_MASK & ((sel as u32) << 19)))
    }
    /// Get test selection.
    #[inline]
    pub fn test_sel(self) -> u8 {
        ((self.0 & Self::TEST_SEL_MASK) >> 19) as u8
    }
    /// Enable test mode.
    #[inline]
    pub fn enable_test(self) -> Self {
        Self(self.0 | Self::TEST_EN)
    }
    /// Disable test mode.
    #[inline]
    pub fn disable_test(self) -> Self {
        Self(self.0 & !Self::TEST_EN)
    }
    /// Check if test mode is enabled.
    #[inline]
    pub fn is_test_enabled(self) -> bool {
        self.0 & Self::TEST_EN != 0
    }
    /// Enable bias selection.
    #[inline]
    pub fn enable_bias_sel(self) -> Self {
        Self(self.0 | Self::BIAS_SEL)
    }
    /// Disable bias selection.
    #[inline]
    pub fn disable_bias_sel(self) -> Self {
        Self(self.0 & !Self::BIAS_SEL)
    }
    /// Check if bias selection is enabled.
    #[inline]
    pub fn is_bias_sel_enabled(self) -> bool {
        self.0 & Self::BIAS_SEL != 0
    }
    /// Set chop mode.
    #[inline]
    pub fn set_chop_mode(self, mode: u8) -> Self {
        Self((self.0 & !Self::CHOP_MODE_MASK) | (Self::CHOP_MODE_MASK & ((mode as u32) << 15)))
    }
    /// Get chop mode.
    #[inline]
    pub fn chop_mode(self) -> u8 {
        ((self.0 & Self::CHOP_MODE_MASK) >> 15) as u8
    }
    /// Enable PGA VCMI.
    #[inline]
    pub fn enable_pga_vcmi(self) -> Self {
        Self(self.0 | Self::PGA_VCMI_EN)
    }
    /// Disable PGA VCMI.
    #[inline]
    pub fn disable_pga_vcmi(self) -> Self {
        Self(self.0 & !Self::PGA_VCMI_EN)
    }
    /// Check if PGA VCMI is enabled.
    #[inline]
    pub fn is_pga_vcmi_enabled(self) -> bool {
        self.0 & Self::PGA_VCMI_EN != 0
    }
    /// Enable PGA.
    #[inline]
    pub fn enable_pga(self) -> Self {
        Self(self.0 | Self::PGA_EN)
    }
    /// Disable PGA.
    #[inline]
    pub fn disable_pga(self) -> Self {
        Self(self.0 & !Self::PGA_EN)
    }
    /// Check if PGA is enabled.
    #[inline]
    pub fn is_pga_enabled(self) -> bool {
        self.0 & Self::PGA_EN != 0
    }
    /// Set PGA offset calibration.
    #[inline]
    pub fn set_pga_os_cal(self, cal: u8) -> Self {
        Self((self.0 & !Self::PGA_OS_CAL_MASK) | (Self::PGA_OS_CAL_MASK & ((cal as u32) << 9)))
    }
    /// Get PGA offset calibration.
    #[inline]
    pub fn pga_os_cal(self) -> u8 {
        ((self.0 & Self::PGA_OS_CAL_MASK) >> 9) as u8
    }
    /// Set PGA VCM.
    #[inline]
    pub fn set_pga_vcm(self, vcm: u8) -> Self {
        Self((self.0 & !Self::PGA_VCM_MASK) | (Self::PGA_VCM_MASK & ((vcm as u32) << 7)))
    }
    /// Get PGA VCM.
    #[inline]
    pub fn pga_vcm(self) -> u8 {
        ((self.0 & Self::PGA_VCM_MASK) >> 7) as u8
    }
    /// Enable temperature sensor.
    #[inline]
    pub fn enable_ts(self) -> Self {
        Self(self.0 | Self::TS_EN)
    }
    /// Disable temperature sensor.
    #[inline]
    pub fn disable_ts(self) -> Self {
        Self(self.0 & !Self::TS_EN)
    }
    /// Check if temperature sensor is enabled.
    #[inline]
    pub fn is_ts_enabled(self) -> bool {
        self.0 & Self::TS_EN != 0
    }
    /// Set external temperature sensor selection.
    #[inline]
    pub fn set_tsext_sel(self, sel: bool) -> Self {
        if sel {
            Self(self.0 | Self::TSEXT_SEL)
        } else {
            Self(self.0 & !Self::TSEXT_SEL)
        }
    }
    /// Check if external temperature sensor is selected.
    #[inline]
    pub fn is_tsext_sel(self) -> bool {
        self.0 & Self::TSEXT_SEL != 0
    }
    /// Enable battery voltage reference.
    #[inline]
    pub fn enable_vbat(self) -> Self {
        Self(self.0 | Self::VBAT_EN)
    }
    /// Disable battery voltage reference.
    #[inline]
    pub fn disable_vbat(self) -> Self {
        Self(self.0 & !Self::VBAT_EN)
    }
    /// Check if battery voltage reference is enabled.
    #[inline]
    pub fn is_vbat_enabled(self) -> bool {
        self.0 & Self::VBAT_EN != 0
    }
    /// Set voltage reference selection.
    #[inline]
    pub fn set_vref_sel(self, sel: bool) -> Self {
        if sel {
            Self(self.0 | Self::VREF_SEL)
        } else {
            Self(self.0 & !Self::VREF_SEL)
        }
    }
    /// Check if voltage reference is selected.
    #[inline]
    pub fn is_vref_sel(self) -> bool {
        self.0 & Self::VREF_SEL != 0
    }
    /// Enable differential mode.
    #[inline]
    pub fn enable_diff_mode(self) -> Self {
        Self(self.0 | Self::DIFF_MODE)
    }
    /// Disable differential mode.
    #[inline]
    pub fn disable_diff_mode(self) -> Self {
        Self(self.0 & !Self::DIFF_MODE)
    }
    /// Check if differential mode is enabled.
    #[inline]
    pub fn is_diff_mode_enabled(self) -> bool {
        self.0 & Self::DIFF_MODE != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpadcChannel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    ChannelDacA,
    ChannelDacB,
    ChannelTSENP,
    ChannelTSENN,
    ChannelVRef,
    ChannelVBatHalf = 18,
    ChannelVGND = 23,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConverationSequence1(u32);

impl GpadcConverationSequence1 {
    const SCAN_POS_5_MASK: u32 = 0x1F << 25;
    const SCAN_POS_4_MASK: u32 = 0x1F << 20;
    const SCAN_POS_3_MASK: u32 = 0x1F << 15;
    const SCAN_POS_2_MASK: u32 = 0x1F << 10;
    const SCAN_POS_1_MASK: u32 = 0x1F << 5;
    const SCAN_POS_0_MASK: u32 = 0x1F << 0;

    /// Set scan postive position 5.
    #[inline]
    pub fn set_scan_pos_5(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_5_MASK) | (Self::SCAN_POS_5_MASK & ((channel as u32) << 25)))
    }
    /// Get scan postive position 5.
    #[inline]
    pub fn scan_pos_5(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_5_MASK) >> 25) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 4.
    #[inline]
    pub fn set_scan_pos_4(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_4_MASK) | (Self::SCAN_POS_4_MASK & ((channel as u32) << 20)))
    }
    /// Get scan postive position 4.
    #[inline]
    pub fn scan_pos_4(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_4_MASK) >> 20) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 3.
    #[inline]
    pub fn set_scan_pos_3(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_3_MASK) | (Self::SCAN_POS_3_MASK & ((channel as u32) << 15)))
    }
    /// Get scan postive position 3.
    #[inline]
    pub fn scan_pos_3(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_3_MASK) >> 15) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 2.
    #[inline]
    pub fn set_scan_pos_2(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_2_MASK) | (Self::SCAN_POS_2_MASK & ((channel as u32) << 10)))
    }
    /// Get scan postive position 2.
    #[inline]
    pub fn scan_pos_2(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_2_MASK) >> 10) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 1.
    #[inline]
    pub fn set_scan_pos_1(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_1_MASK) | (Self::SCAN_POS_1_MASK & ((channel as u32) << 5)))
    }
    /// Get scan postive position 1.
    #[inline]
    pub fn scan_pos_1(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_1_MASK) >> 5) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 0.
    #[inline]
    pub fn set_scan_pos_0(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_0_MASK) | (Self::SCAN_POS_0_MASK & (channel as u32)))
    }
    /// Get scan postive position 0.
    #[inline]
    pub fn scan_pos_0(self) -> GpadcChannel {
        match (self.0 & Self::SCAN_POS_0_MASK) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConverationSequence2(u32);

impl GpadcConverationSequence2 {
    const SCAN_POS_11_MASK: u32 = 0x1F << 25;
    const SCAN_POS_10_MASK: u32 = 0x1F << 20;
    const SCAN_POS_9_MASK: u32 = 0x1F << 15;
    const SCAN_POS_8_MASK: u32 = 0x1F << 10;
    const SCAN_POS_7_MASK: u32 = 0x1F << 5;
    const SCAN_POS_6_MASK: u32 = 0x1F << 0;

    /// Set scan postive position 11.
    #[inline]
    pub fn set_scan_pos_11(self, channel: GpadcChannel) -> Self {
        Self(
            (self.0 & !Self::SCAN_POS_11_MASK)
                | (Self::SCAN_POS_11_MASK & ((channel as u32) << 25)),
        )
    }
    /// Get scan postive position 11.
    #[inline]
    pub fn scan_pos_11(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_11_MASK) >> 25) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 10.
    #[inline]
    pub fn set_scan_pos_10(self, channel: GpadcChannel) -> Self {
        Self(
            (self.0 & !Self::SCAN_POS_10_MASK)
                | (Self::SCAN_POS_10_MASK & ((channel as u32) << 20)),
        )
    }
    /// Get scan postive position 10.
    #[inline]
    pub fn scan_pos_10(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_10_MASK) >> 20) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 9.
    #[inline]
    pub fn set_scan_pos_9(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_9_MASK) | (Self::SCAN_POS_9_MASK & ((channel as u32) << 15)))
    }
    /// Get scan postive position 9.
    #[inline]
    pub fn scan_pos_9(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_9_MASK) >> 15) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 8.
    #[inline]
    pub fn set_scan_pos_8(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_8_MASK) | (Self::SCAN_POS_8_MASK & ((channel as u32) << 10)))
    }
    /// Get scan postive position 8.
    #[inline]
    pub fn scan_pos_8(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_8_MASK) >> 10) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 7.
    #[inline]
    pub fn set_scan_pos_7(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_7_MASK) | (Self::SCAN_POS_7_MASK & ((channel as u32) << 5)))
    }
    /// Get scan postive position 7.
    #[inline]
    pub fn scan_pos_7(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_POS_7_MASK) >> 5) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan postive position 6.
    #[inline]
    pub fn set_scan_pos_6(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_POS_6_MASK) | (Self::SCAN_POS_6_MASK & (channel as u32)))
    }
    /// Get scan postive position 6.
    #[inline]
    pub fn scan_pos_6(self) -> GpadcChannel {
        match (self.0 & Self::SCAN_POS_6_MASK) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConverationSequence3(u32);

impl GpadcConverationSequence3 {
    const SCAN_NEG_5_MASK: u32 = 0x1F << 25;
    const SCAN_NEG_4_MASK: u32 = 0x1F << 20;
    const SCAN_NEG_3_MASK: u32 = 0x1F << 15;
    const SCAN_NEG_2_MASK: u32 = 0x1F << 10;
    const SCAN_NEG_1_MASK: u32 = 0x1F << 5;
    const SCAN_NEG_0_MASK: u32 = 0x1F << 0;

    /// Set scan negative position 5.
    #[inline]
    pub fn set_scan_neg_5(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_5_MASK) | (Self::SCAN_NEG_5_MASK & ((channel as u32) << 25)))
    }
    /// Get scan negative position 5.
    #[inline]
    pub fn scan_neg_5(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_5_MASK) >> 25) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 4.
    #[inline]
    pub fn set_scan_neg_4(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_4_MASK) | (Self::SCAN_NEG_4_MASK & ((channel as u32) << 20)))
    }
    /// Get scan negative position 4.
    #[inline]
    pub fn scan_neg_4(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_4_MASK) >> 20) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 3.
    #[inline]
    pub fn set_scan_neg_3(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_3_MASK) | (Self::SCAN_NEG_3_MASK & ((channel as u32) << 15)))
    }
    /// Get scan negative position 3.
    #[inline]
    pub fn scan_neg_3(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_3_MASK) >> 15) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 2.
    #[inline]
    pub fn set_scan_neg_2(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_2_MASK) | (Self::SCAN_NEG_2_MASK & ((channel as u32) << 10)))
    }
    /// Get scan negative position 2.
    #[inline]
    pub fn scan_neg_2(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_2_MASK) >> 10) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 1.
    #[inline]
    pub fn set_scan_neg_1(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_1_MASK) | (Self::SCAN_NEG_1_MASK & ((channel as u32) << 5)))
    }
    /// Get scan negative position 1.
    #[inline]
    pub fn scan_neg_1(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_1_MASK) >> 5) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 0.
    #[inline]
    pub fn set_scan_neg_0(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_0_MASK) | (Self::SCAN_NEG_0_MASK & (channel as u32)))
    }
    /// Get scan negative position 0.
    #[inline]
    pub fn scan_neg_0(self) -> GpadcChannel {
        match (self.0 & Self::SCAN_NEG_0_MASK) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConverationSequence4(u32);

impl GpadcConverationSequence4 {
    const SCAN_NEG_11_MASK: u32 = 0x1F << 25;
    const SCAN_NEG_10_MASK: u32 = 0x1F << 20;
    const SCAN_NEG_9_MASK: u32 = 0x1F << 15;
    const SCAN_NEG_8_MASK: u32 = 0x1F << 10;
    const SCAN_NEG_7_MASK: u32 = 0x1F << 5;
    const SCAN_NEG_6_MASK: u32 = 0x1F << 0;

    /// Set scan negative position 11.
    #[inline]
    pub fn set_scan_neg_11(self, channel: GpadcChannel) -> Self {
        Self(
            (self.0 & !Self::SCAN_NEG_11_MASK)
                | (Self::SCAN_NEG_11_MASK & ((channel as u32) << 25)),
        )
    }
    /// Get scan negative position 11.
    #[inline]
    pub fn scan_neg_11(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_11_MASK) >> 25) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 10.
    #[inline]
    pub fn set_scan_neg_10(self, channel: GpadcChannel) -> Self {
        Self(
            (self.0 & !Self::SCAN_NEG_10_MASK)
                | (Self::SCAN_NEG_10_MASK & ((channel as u32) << 20)),
        )
    }
    /// Get scan negative position 10.
    #[inline]
    pub fn scan_neg_10(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_10_MASK) >> 20) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 9.
    #[inline]
    pub fn set_scan_neg_9(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_9_MASK) | (Self::SCAN_NEG_9_MASK & ((channel as u32) << 15)))
    }
    /// Get scan negative position 9.
    #[inline]
    pub fn scan_neg_9(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_9_MASK) >> 15) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 8.
    #[inline]
    pub fn set_scan_neg_8(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_8_MASK) | (Self::SCAN_NEG_8_MASK & ((channel as u32) << 10)))
    }
    /// Get scan negative position 8.
    #[inline]
    pub fn scan_neg_8(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_8_MASK) >> 10) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 7.
    #[inline]
    pub fn set_scan_neg_7(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_7_MASK) | (Self::SCAN_NEG_7_MASK & ((channel as u32) << 5)))
    }
    /// Get scan negative position 7.
    #[inline]
    pub fn scan_neg_7(self) -> GpadcChannel {
        match ((self.0 & Self::SCAN_NEG_7_MASK) >> 5) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
    /// Set scan negative position 6.
    #[inline]
    pub fn set_scan_neg_6(self, channel: GpadcChannel) -> Self {
        Self((self.0 & !Self::SCAN_NEG_6_MASK) | (Self::SCAN_NEG_6_MASK & (channel as u32)))
    }
    /// Get scan negative position 6.
    #[inline]
    pub fn scan_neg_6(self) -> GpadcChannel {
        match (self.0 & Self::SCAN_NEG_6_MASK) as u8 {
            0 => GpadcChannel::Channel0,
            1 => GpadcChannel::Channel1,
            2 => GpadcChannel::Channel2,
            3 => GpadcChannel::Channel3,
            4 => GpadcChannel::Channel4,
            5 => GpadcChannel::Channel5,
            6 => GpadcChannel::Channel6,
            7 => GpadcChannel::Channel7,
            8 => GpadcChannel::Channel8,
            9 => GpadcChannel::Channel9,
            10 => GpadcChannel::Channel10,
            11 => GpadcChannel::Channel11,
            12 => GpadcChannel::ChannelDacA,
            13 => GpadcChannel::ChannelDacB,
            14 => GpadcChannel::ChannelTSENP,
            15 => GpadcChannel::ChannelTSENN,
            16 => GpadcChannel::ChannelVRef,
            18 => GpadcChannel::ChannelVBatHalf,
            23 => GpadcChannel::ChannelVGND,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcStatus(u32);

impl GpadcStatus {
    const RESERVED_MASK: u32 = 0xffff << 16;
    const DATA_RDY: u32 = 0x1 << 0;

    /// Check if data is ready.
    #[inline]
    pub fn is_data_ready(self) -> bool {
        self.0 & Self::DATA_RDY != 0
    }
    /// Get reserved bits.
    #[inline]
    pub fn reserved(self) -> u16 {
        ((self.0 & Self::RESERVED_MASK) >> 16) as u16
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcInterruptState(u32);

impl GpadcInterruptState {
    const POS_SATUR_MASK: u32 = 0x1 << 9;
    const NEG_SATUR_MASK: u32 = 0x1 << 8;
    const POS_SATUR_CLR: u32 = 0x1 << 5;
    const NEG_SATUR_CLR: u32 = 0x1 << 4;
    const POS_SATUR: u32 = 0x1 << 1;
    const NEG_SATUR: u32 = 0x1 << 0;

    /// Enable positive saturation interrupt.
    #[inline]
    pub fn enable_pos_satur_interrupt(self) -> Self {
        Self(self.0 & !Self::POS_SATUR_MASK)
    }
    /// Disable positive saturation interrupt.
    #[inline]
    pub fn disable_pos_satur_interrupt(self) -> Self {
        Self(self.0 | Self::POS_SATUR_MASK)
    }
    /// Check if positive saturation interrupt is enabled.
    #[inline]
    pub fn is_pos_satur_interrupt_enabled(self) -> bool {
        self.0 & Self::POS_SATUR_MASK == 0
    }
    /// Enable negative saturation interrupt.
    #[inline]
    pub fn enable_neg_satur_interrupt(self) -> Self {
        Self(self.0 & !Self::NEG_SATUR_MASK)
    }
    /// Disable negative saturation interrupt.
    #[inline]
    pub fn disable_neg_satur_interrupt(self) -> Self {
        Self(self.0 | Self::NEG_SATUR_MASK)
    }
    /// Check if negative saturation interrupt is enabled.
    #[inline]
    pub fn is_neg_satur_interrupt_enabled(self) -> bool {
        self.0 & Self::NEG_SATUR_MASK == 0
    }
    /// Set positive saturation interrupt bit.
    #[inline]
    pub fn set_pos_satur_interrupt(self, set: bool) -> Self {
        Self(if set {
            self.0 | Self::POS_SATUR
        } else {
            self.0 & !Self::POS_SATUR
        })
    }
    /// Clear positive saturation interrupt.
    #[inline]
    pub fn clear_pos_satur_interrupt(self) -> Self {
        Self(self.0 | Self::POS_SATUR_CLR)
    }
    /// Set negative saturation interrupt bit.
    #[inline]
    pub fn set_neg_satur_interrupt(self, set: bool) -> Self {
        Self(if set {
            self.0 | Self::NEG_SATUR
        } else {
            self.0 & !Self::NEG_SATUR
        })
    }
    /// Clear negative saturation interrupt.
    #[inline]
    pub fn clear_neg_satur_interrupt(self) -> Self {
        Self(self.0 | Self::NEG_SATUR_CLR)
    }
    /// Check if positive saturation interrupt occurs.
    #[inline]
    pub fn if_pos_satur_interrupt_occurs(self) -> bool {
        self.0 & Self::POS_SATUR != 0
    }
    /// Check if negative saturation interrupt occurs.
    #[inline]
    pub fn if_neg_satur_interrupt_occurs(self) -> bool {
        self.0 & Self::NEG_SATUR != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcResult(u32);

impl GpadcResult {
    const DATA_OUT_MASK: u32 = 0x3ffffff << 0;

    /// Get the ADC data output.
    #[inline]
    pub fn data_out(self) -> u32 {
        self.0 & Self::DATA_OUT_MASK
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcRawResult(u32);

impl GpadcRawResult {
    const RAW_DATA_MASK: u32 = 0xfff << 0;

    /// Get the raw ADC data.
    #[inline]
    pub fn raw_data(self) -> u16 {
        (self.0 & Self::RAW_DATA_MASK) as u16
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcDefine(u32);

impl GpadcDefine {
    const OS_CAL_DATA_MASK: u32 = 0xffff << 0;

    /// Set the offset calibration data.
    #[inline]
    pub fn set_os_cal_data(self, data: u16) -> Self {
        Self((self.0 & !Self::OS_CAL_DATA_MASK) | ((data as u32) & Self::OS_CAL_DATA_MASK))
    }
    /// Get the offset calibration data.
    #[inline]
    pub fn os_cal_data(self) -> u16 {
        (self.0 & Self::OS_CAL_DATA_MASK) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GpadcChannel, GpadcClkDivider, GpadcCommand, GpadcConfig1, GpadcConfig2,
        GpadcConverationSequence1, GpadcConverationSequence2, GpadcConverationSequence3,
        GpadcConverationSequence4, GpadcDefine, GpadcInterruptState, GpadcRawResult,
        GpadcResolution, GpadcResult, GpadcStatus, RegisterBlock,
    };
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, time_lo), 0x04);
        assert_eq!(offset_of!(RegisterBlock, time_hi), 0x08);
        assert_eq!(offset_of!(RegisterBlock, rtc_time_lo), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, rtc_time_hi), 0x10);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mode), 0x14);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state), 0x18);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, pir_config), 0x20);
        assert_eq!(offset_of!(RegisterBlock, pir_threshold), 0x24);
        assert_eq!(offset_of!(RegisterBlock, pir_interval), 0x28);
        assert_eq!(offset_of!(RegisterBlock, bor_config), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, global), 0x30);
        assert_eq!(offset_of!(RegisterBlock, sram), 0x34);
        assert_eq!(offset_of!(RegisterBlock, pad_control_0), 0x38);
        assert_eq!(offset_of!(RegisterBlock, pad_control_1), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, rc32k), 0x200);
        assert_eq!(offset_of!(RegisterBlock, xtal32k), 0x204);
        assert_eq!(offset_of!(RegisterBlock, rtc_control_0), 0x208);
        assert_eq!(offset_of!(RegisterBlock, rtc_control_1), 0x20c);
        assert_eq!(offset_of!(RegisterBlock, gpadc_command), 0x90C);
        assert_eq!(offset_of!(RegisterBlock, gpadc_config_1), 0x910);
        assert_eq!(offset_of!(RegisterBlock, gpadc_config_2), 0x914);
        assert_eq!(
            offset_of!(RegisterBlock, gpadc_converation_sequence_1),
            0x918
        );
        assert_eq!(
            offset_of!(RegisterBlock, gpadc_converation_sequence_2),
            0x91C
        );
        assert_eq!(
            offset_of!(RegisterBlock, gpadc_converation_sequence_3),
            0x920
        );
        assert_eq!(
            offset_of!(RegisterBlock, gpadc_converation_sequence_4),
            0x924
        );
        assert_eq!(offset_of!(RegisterBlock, gpadc_status), 0x928);
        assert_eq!(offset_of!(RegisterBlock, gpadc_interrupt_state), 0x92C);
        assert_eq!(offset_of!(RegisterBlock, gpadc_result), 0x930);
        assert_eq!(offset_of!(RegisterBlock, gpadc_raw_result), 0x934);
        assert_eq!(offset_of!(RegisterBlock, gpadc_define), 0x938);
    }

    #[test]
    fn struct_gpadc_command_functions() {
        let mut val = GpadcCommand(0);

        val = val.enable_sensor_test_v2();
        assert!(val.is_sensor_test_v2_enabled());
        assert_eq!(val.0, 0x8000_0000);

        val = val.disable_sensor_test_v2();
        assert!(!val.is_sensor_test_v2_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_sensor_test_v1();
        assert!(val.is_sensor_test_v1_enabled());
        assert_eq!(val.0, 0x4000_0000);

        val = val.disable_sensor_test_v1();
        assert!(!val.is_sensor_test_v1_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_sensor_v1(2);
        assert_eq!(val.sensor_v1(), 2);
        assert_eq!(val.0, 0x2000_0000);

        val = val.set_sensor_v2(3);
        assert_eq!(val.sensor_v2(), 3);
        assert_eq!(val.0, 0x3000_0000);

        val = GpadcCommand(0).enable_chip_sen_pu();
        assert!(val.is_chip_sen_pu_enabled());
        assert_eq!(val.0, 0x0800_0000);

        val = val.disable_chip_sen_pu();
        assert!(!val.is_chip_sen_pu_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_micboost_32db();
        assert!(val.is_micboost_32db_enabled());
        assert_eq!(val.0, 0x0080_0000);

        val = val.disable_micboost_32db();
        assert!(!val.is_micboost_32db_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_mic_pga2_gain(2);
        assert_eq!(val.mic_pga2_gain(), 2);
        assert_eq!(val.0, 0x0040_0000);

        val = GpadcCommand(0).enable_mic1_diff();
        assert!(val.is_mic1_diff_enabled());
        assert_eq!(val.0, 0x0010_0000);

        val = val.disable_mic1_diff();
        assert!(!val.is_mic1_diff_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_mic2_diff();
        assert!(val.is_mic2_diff_enabled());
        assert_eq!(val.0, 0x0008_0000);

        val = val.disable_mic2_diff();
        assert!(!val.is_mic2_diff_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dwa();
        assert!(val.is_dwa_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_dwa();
        assert!(!val.is_dwa_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rcal();
        assert!(val.is_rcal_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = val.disable_rcal();
        assert!(!val.is_rcal_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_byp_micboost();
        assert!(val.is_byp_micboost_enabled());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_byp_micboost();
        assert!(!val.is_byp_micboost_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_micpga();
        assert!(val.is_micpga_enabled());
        assert_eq!(val.0, 0x0000_8000);

        val = val.disable_micpga();
        assert!(!val.is_micpga_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_micbias();
        assert!(val.is_micbias_enabled());
        assert_eq!(val.0, 0x0000_4000);

        val = val.disable_micbias();
        assert!(!val.is_micbias_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_neg_gnd();
        assert!(val.is_neg_gnd_set());
        assert_eq!(val.0, 0x0000_2000);

        val = val.unset_neg_gnd();
        assert!(!val.is_neg_gnd_set());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..19 as u8 {
            let channel = match i {
                0 => GpadcChannel::Channel0,
                1 => GpadcChannel::Channel1,
                2 => GpadcChannel::Channel2,
                3 => GpadcChannel::Channel3,
                4 => GpadcChannel::Channel4,
                5 => GpadcChannel::Channel5,
                6 => GpadcChannel::Channel6,
                7 => GpadcChannel::Channel7,
                8 => GpadcChannel::Channel8,
                9 => GpadcChannel::Channel9,
                10 => GpadcChannel::Channel10,
                11 => GpadcChannel::Channel11,
                12 => GpadcChannel::ChannelDacA,
                13 => GpadcChannel::ChannelDacB,
                14 => GpadcChannel::ChannelTSENP,
                15 => GpadcChannel::ChannelTSENN,
                16 => GpadcChannel::ChannelVRef,
                17 => GpadcChannel::ChannelVBatHalf,
                18 => GpadcChannel::ChannelVGND,
                _ => unreachable!(),
            };

            let val_tmp = match i {
                0 => 0x0000_0000,
                1 => 0x0000_0100,
                2 => 0x0000_0200,
                3 => 0x0000_0300,
                4 => 0x0000_0400,
                5 => 0x0000_0500,
                6 => 0x0000_0600,
                7 => 0x0000_0700,
                8 => 0x0000_0800,
                9 => 0x0000_0900,
                10 => 0x0000_0A00,
                11 => 0x0000_0B00,
                12 => 0x0000_0C00,
                13 => 0x0000_0D00,
                14 => 0x0000_0E00,
                15 => 0x0000_0F00,
                16 => 0x0000_1000,
                17 => 0x0000_1200,
                18 => 0x0000_1700,
                _ => unreachable!(),
            };

            let mut val = GpadcCommand(0);
            val = val.set_pos_sel(channel);
            assert_eq!(val.pos_sel(), channel);
            assert_eq!(val.0, val_tmp);

            let val_tmp = match i {
                0 => 0x0000_0000,
                1 => 0x0000_0008,
                2 => 0x0000_0010,
                3 => 0x0000_0018,
                4 => 0x0000_0020,
                5 => 0x0000_0028,
                6 => 0x0000_0030,
                7 => 0x0000_0038,
                8 => 0x0000_0040,
                9 => 0x0000_0048,
                10 => 0x0000_0050,
                11 => 0x0000_0058,
                12 => 0x0000_0060,
                13 => 0x0000_0068,
                14 => 0x0000_0070,
                15 => 0x0000_0078,
                16 => 0x0000_0080,
                17 => 0x0000_0090,
                18 => 0x0000_00B8,
                _ => unreachable!(),
            };

            val = GpadcCommand(0);
            val = val.set_neg_sel(channel);
            assert_eq!(val.neg_sel(), channel);
            assert_eq!(val.0, val_tmp);
        }

        val = GpadcCommand(0).start_software_reset();
        assert_eq!(val.0, 0x0000_0004);

        val = val.stop_software_reset();
        assert_eq!(val.0, 0x0000_0000);

        val = val.start_conversion();
        assert_eq!(val.0, 0x0000_0002);

        val = val.stop_conversion();
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_global();
        assert!(val.is_global_enabled());
        assert_eq!(val.0, 0x0000_0001);

        val = val.disable_global();
        assert!(!val.is_global_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_config1_functions() {
        let mut val = GpadcConfig1(0);

        val = val.set_v18_sel(2);
        assert_eq!(val.v18_sel(), 2);
        assert_eq!(val.0, 0x4000_0000);

        val = GpadcConfig1(0).set_v11_sel(0x3);
        assert_eq!(val.v11_sel(), 3);
        assert_eq!(val.0, 0x1800_0000);

        val = GpadcConfig1(0).enable_dither();
        assert!(val.is_dither_enabled());
        assert_eq!(val.0, 0x0400_0000);

        val = val.disable_dither();
        assert!(!val.is_dither_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_scan();
        assert!(val.is_scan_enabled());
        assert_eq!(val.0, 0x0200_0000);

        val = val.disable_scan();
        assert!(!val.is_scan_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_scan_length(0xF);
        assert_eq!(val.scan_length(), 0xF);
        assert_eq!(val.0, 0x01E0_0000);

        val = GpadcConfig1(0).set_clk_div_ratio(GpadcClkDivider::Div4);
        assert_eq!(val.clk_div_ratio(), GpadcClkDivider::Div4);
        assert_eq!(val.0, 0x0004_0000);

        val = val.set_clk_div_ratio(GpadcClkDivider::Div32);
        assert_eq!(val.clk_div_ratio(), GpadcClkDivider::Div32);
        assert_eq!(val.0, 0x001C_0000);

        val = GpadcConfig1(0).enable_clk_ana_inv();
        assert!(val.is_clk_ana_inv_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = val.disable_clk_ana_inv();
        assert!(!val.is_clk_ana_inv_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_clk_ana_dly();
        assert!(val.is_clk_ana_dly_enabled());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_clk_ana_dly();
        assert!(!val.is_clk_ana_dly_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_ana_dly(0xF);
        assert_eq!(val.clk_ana_dly(), 0xF);
        assert_eq!(val.0, 0x0000_F000);

        val = GpadcConfig1(0).enable_pwm_trigger();
        assert!(val.is_pwm_trigger_enabled());
        assert_eq!(val.0, 0x0000_0800);

        val = val.disable_pwm_trigger();
        assert!(!val.is_pwm_trigger_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_lowv_det();
        assert!(val.is_lowv_det_enabled());
        assert_eq!(val.0, 0x0000_0400);

        val = val.disable_lowv_det();
        assert!(!val.is_lowv_det_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_vcm_hyst_sel();
        assert!(val.is_vcm_hyst_sel_enabled());
        assert_eq!(val.0, 0x0000_0200);

        val = val.disable_vcm_hyst_sel();
        assert!(!val.is_vcm_hyst_sel_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_vcm_sel();
        assert!(val.is_vcm_sel_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_vcm_sel();
        assert!(!val.is_vcm_sel_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_res_sel(GpadcResolution::Bit12);
        assert_eq!(val.res_sel(), GpadcResolution::Bit12);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_res_sel(GpadcResolution::Bit14);
        assert_eq!(val.res_sel(), GpadcResolution::Bit14);
        assert_eq!(val.0, 0x0000_0008);

        val = val.set_res_sel(GpadcResolution::Bit16);
        assert_eq!(val.res_sel(), GpadcResolution::Bit16);
        assert_eq!(val.0, 0x0000_0010);

        val = GpadcConfig1(0).enable_continuous_conv();
        assert!(val.is_continuous_conv_enabled());
        assert_eq!(val.0, 0x0000_0002);

        val = val.disable_continuous_conv();
        assert!(!val.is_continuous_conv_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cal_os();
        assert!(val.is_cal_os_enabled());
        assert_eq!(val.0, 0x0000_0001);

        val = val.disable_cal_os();
        assert!(!val.is_cal_os_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_config2_functions() {
        let mut val = GpadcConfig2(0);

        val = val.enable_tsvbe_low();
        assert!(val.is_tsvbe_low_enabled());
        assert_eq!(val.0, 0x8000_0000);

        val = val.disable_tsvbe_low();
        assert!(!val.is_tsvbe_low_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_dly_sel(7);
        assert_eq!(val.dly_sel(), 7);
        assert_eq!(val.0, 0x7000_0000);

        val = GpadcConfig2(0).set_pga1_gain(7);
        assert_eq!(val.pga1_gain(), 7);
        assert_eq!(val.0, 0x0E00_0000);

        val = GpadcConfig2(0).set_pga2_gain(7);
        assert_eq!(val.pga2_gain(), 7);
        assert_eq!(val.0, 0x01C0_0000);

        val = GpadcConfig2(0).set_test_sel(7);
        assert_eq!(val.test_sel(), 7);
        assert_eq!(val.0, 0x0038_0000);

        val = GpadcConfig2(0).enable_test();
        assert!(val.is_test_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_test();
        assert!(!val.is_test_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_bias_sel();
        assert!(val.is_bias_sel_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = val.disable_bias_sel();
        assert!(!val.is_bias_sel_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_chop_mode(3);
        assert_eq!(val.chop_mode(), 3);
        assert_eq!(val.0, 0x0001_8000);

        val = GpadcConfig2(0).enable_pga_vcmi();
        assert!(val.is_pga_vcmi_enabled());
        assert_eq!(val.0, 0x0000_4000);

        val = val.disable_pga_vcmi();
        assert!(!val.is_pga_vcmi_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_pga();
        assert!(val.is_pga_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = val.disable_pga();
        assert!(!val.is_pga_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = GpadcConfig2(0).set_pga_os_cal(0xF);
        assert_eq!(val.pga_os_cal(), 0xF);
        assert_eq!(val.0, 0x0000_1E00);

        val = GpadcConfig2(0).set_pga_vcm(3);
        assert_eq!(val.pga_vcm(), 3);
        assert_eq!(val.0, 0x0000_0180);

        val = GpadcConfig2(0).enable_ts();
        assert!(val.is_ts_enabled());
        assert_eq!(val.0, 0x0000_0040);

        val = val.disable_ts();
        assert!(!val.is_ts_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_tsext_sel(true);
        assert!(val.is_tsext_sel());
        assert_eq!(val.0, 0x0000_0020);

        val = val.set_tsext_sel(false);
        assert!(!val.is_tsext_sel());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_vbat();
        assert!(val.is_vbat_enabled());
        assert_eq!(val.0, 0x0000_0010);

        val = val.disable_vbat();
        assert!(!val.is_vbat_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_vref_sel(true);
        assert!(val.is_vref_sel());
        assert_eq!(val.0, 0x0000_0008);

        val = val.set_vref_sel(false);
        assert!(!val.is_vref_sel());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_diff_mode();
        assert!(val.is_diff_mode_enabled());
        assert_eq!(val.0, 0x0000_0004);

        val = val.disable_diff_mode();
        assert!(!val.is_diff_mode_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_adc_converation_sequence_functions() {
        for i in 0..19 as u8 {
            let channel = match i {
                0 => GpadcChannel::Channel0,
                1 => GpadcChannel::Channel1,
                2 => GpadcChannel::Channel2,
                3 => GpadcChannel::Channel3,
                4 => GpadcChannel::Channel4,
                5 => GpadcChannel::Channel5,
                6 => GpadcChannel::Channel6,
                7 => GpadcChannel::Channel7,
                8 => GpadcChannel::Channel8,
                9 => GpadcChannel::Channel9,
                10 => GpadcChannel::Channel10,
                11 => GpadcChannel::Channel11,
                12 => GpadcChannel::ChannelDacA,
                13 => GpadcChannel::ChannelDacB,
                14 => GpadcChannel::ChannelTSENP,
                15 => GpadcChannel::ChannelTSENN,
                16 => GpadcChannel::ChannelVRef,
                17 => GpadcChannel::ChannelVBatHalf,
                18 => GpadcChannel::ChannelVGND,
                _ => unreachable!(),
            };

            // GpadcConverationSequence1 pos 0-5
            let mut val = GpadcConverationSequence1(0);

            // Test pos 0
            val = val.set_scan_pos_0(channel);
            assert_eq!(val.scan_pos_0(), channel);
            assert_eq!(val.0, (channel as u32) << 0);

            // Test pos 1
            val = GpadcConverationSequence1(0).set_scan_pos_1(channel);
            assert_eq!(val.scan_pos_1(), channel);
            assert_eq!(val.0, (channel as u32) << 5);

            // Test pos 2
            val = GpadcConverationSequence1(0).set_scan_pos_2(channel);
            assert_eq!(val.scan_pos_2(), channel);
            assert_eq!(val.0, (channel as u32) << 10);

            // Test pos 3
            val = GpadcConverationSequence1(0).set_scan_pos_3(channel);
            assert_eq!(val.scan_pos_3(), channel);
            assert_eq!(val.0, (channel as u32) << 15);

            // Test pos 4
            val = GpadcConverationSequence1(0).set_scan_pos_4(channel);
            assert_eq!(val.scan_pos_4(), channel);
            assert_eq!(val.0, (channel as u32) << 20);

            // Test pos 5
            val = GpadcConverationSequence1(0).set_scan_pos_5(channel);
            assert_eq!(val.scan_pos_5(), channel);
            assert_eq!(val.0, (channel as u32) << 25);

            // GpadcConverationSequence2 pos 6-11
            let mut val = GpadcConverationSequence2(0);

            // Test pos 6
            val = val.set_scan_pos_6(channel);
            assert_eq!(val.scan_pos_6(), channel);
            assert_eq!(val.0, (channel as u32) << 0);

            // Test pos 7
            val = GpadcConverationSequence2(0).set_scan_pos_7(channel);
            assert_eq!(val.scan_pos_7(), channel);
            assert_eq!(val.0, (channel as u32) << 5);

            // Test pos 8
            val = GpadcConverationSequence2(0).set_scan_pos_8(channel);
            assert_eq!(val.scan_pos_8(), channel);
            assert_eq!(val.0, (channel as u32) << 10);

            // Test pos 9
            val = GpadcConverationSequence2(0).set_scan_pos_9(channel);
            assert_eq!(val.scan_pos_9(), channel);
            assert_eq!(val.0, (channel as u32) << 15);

            // Test pos 10
            val = GpadcConverationSequence2(0).set_scan_pos_10(channel);
            assert_eq!(val.scan_pos_10(), channel);
            assert_eq!(val.0, (channel as u32) << 20);

            // Test pos 11
            val = GpadcConverationSequence2(0).set_scan_pos_11(channel);
            assert_eq!(val.scan_pos_11(), channel);
            assert_eq!(val.0, (channel as u32) << 25);

            // GpadcConverationSequence3 neg 0-5
            let mut val = GpadcConverationSequence3(0);

            // Test neg 0
            val = val.set_scan_neg_0(channel);
            assert_eq!(val.scan_neg_0(), channel);
            assert_eq!(val.0, (channel as u32) << 0);

            // Test neg 1
            val = GpadcConverationSequence3(0).set_scan_neg_1(channel);
            assert_eq!(val.scan_neg_1(), channel);
            assert_eq!(val.0, (channel as u32) << 5);

            // Test neg 2
            val = GpadcConverationSequence3(0).set_scan_neg_2(channel);
            assert_eq!(val.scan_neg_2(), channel);
            assert_eq!(val.0, (channel as u32) << 10);

            // Test neg 3
            val = GpadcConverationSequence3(0).set_scan_neg_3(channel);
            assert_eq!(val.scan_neg_3(), channel);
            assert_eq!(val.0, (channel as u32) << 15);

            // Test neg 4
            val = GpadcConverationSequence3(0).set_scan_neg_4(channel);
            assert_eq!(val.scan_neg_4(), channel);
            assert_eq!(val.0, (channel as u32) << 20);

            // Test neg 5
            val = GpadcConverationSequence3(0).set_scan_neg_5(channel);
            assert_eq!(val.scan_neg_5(), channel);
            assert_eq!(val.0, (channel as u32) << 25);

            // GpadcConverationSequence4 neg 6-11
            let mut val = GpadcConverationSequence4(0);

            // Test neg 6
            val = val.set_scan_neg_6(channel);
            assert_eq!(val.scan_neg_6(), channel);
            assert_eq!(val.0, (channel as u32) << 0);

            // Test neg 7
            val = GpadcConverationSequence4(0).set_scan_neg_7(channel);
            assert_eq!(val.scan_neg_7(), channel);
            assert_eq!(val.0, (channel as u32) << 5);

            // Test neg 8
            val = GpadcConverationSequence4(0).set_scan_neg_8(channel);
            assert_eq!(val.scan_neg_8(), channel);
            assert_eq!(val.0, (channel as u32) << 10);

            // Test neg 9
            val = GpadcConverationSequence4(0).set_scan_neg_9(channel);
            assert_eq!(val.scan_neg_9(), channel);
            assert_eq!(val.0, (channel as u32) << 15);

            // Test neg 10
            val = GpadcConverationSequence4(0).set_scan_neg_10(channel);
            assert_eq!(val.scan_neg_10(), channel);
            assert_eq!(val.0, (channel as u32) << 20);

            // Test neg 11
            val = GpadcConverationSequence4(0).set_scan_neg_11(channel);
            assert_eq!(val.scan_neg_11(), channel);
            assert_eq!(val.0, (channel as u32) << 25);
        }
    }

    #[test]
    fn struct_gpadc_status_functions() {
        let val = GpadcStatus(0x0000_0001);
        assert!(val.is_data_ready());

        let val = GpadcStatus(0xFFFF_0000);
        assert_eq!(val.reserved(), 0xFFFF);
    }

    #[test]
    fn struct_gpadc_interrupt_state_functions() {
        let mut val = GpadcInterruptState(0);

        val = val.enable_pos_satur_interrupt();
        assert!(val.is_pos_satur_interrupt_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_pos_satur_interrupt();
        assert!(!val.is_pos_satur_interrupt_enabled());
        assert_eq!(val.0, 0x0000_0200);

        val = GpadcInterruptState(0).enable_neg_satur_interrupt();
        assert!(val.is_neg_satur_interrupt_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_neg_satur_interrupt();
        assert!(!val.is_neg_satur_interrupt_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = GpadcInterruptState(0).clear_pos_satur_interrupt();
        assert_eq!(val.0, 0x0000_0020);

        val = GpadcInterruptState(0).clear_neg_satur_interrupt();
        assert_eq!(val.0, 0x0000_0010);

        val = GpadcInterruptState(0x0000_0002);
        assert!(val.if_pos_satur_interrupt_occurs());

        val = GpadcInterruptState(0x0000_0001);
        assert!(val.if_neg_satur_interrupt_occurs());
    }

    #[test]
    fn struct_gpadc_result_functions() {
        let val = GpadcResult(0x03FF_FFFF);
        assert_eq!(val.data_out(), 0x03FF_FFFF);

        let val = GpadcResult(0);
        assert_eq!(val.data_out(), 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_raw_result_functions() {
        let val = GpadcRawResult(0x0000_0FFF);
        assert_eq!(val.raw_data(), 0x0FFF);

        let val = GpadcRawResult(0);
        assert_eq!(val.raw_data(), 0x0000_0000);
    }

    #[test]
    fn struct_gpadc_define_functions() {
        let mut val = GpadcDefine(0);

        val = val.set_os_cal_data(0xFFFF);
        assert_eq!(val.os_cal_data(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);

        val = val.set_os_cal_data(0);
        assert_eq!(val.os_cal_data(), 0x0000_0000);
        assert_eq!(val.0, 0x0000_0000);
    }
}
