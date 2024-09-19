//! Generic DAC, ADC and ACOMP interface control peripheral.

use volatile_register::RW;

/// Generic DAC, ADC and ACOMP interface control peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Generic Analog-to-Digital Converter configuration register.
    pub gpadc_config: RW<GpadcConfig>,
    pub gpadc_dma_rdata: RW<GpadcDmaRdata>,
    _reserved0: [u8; 24],
    pub gpadc_pir_train: RW<GpadcPirTrain>,
    _reserved1: [u8; 28],
    pub gpdac_config: RW<GpdacConfig>,
    pub gpdac_dma_config: RW<GpdacDmaConfig>,
    pub gpdac_dma_wdata: RW<GpdacDmaWdata>,
    pub gpdac_tx_fifo_status: RW<GpdacTxFifoStatus>,
    _reserved2: [u8; 696],
    pub gpdac_ctrl: RW<GpdacCtrl>,
    pub gpdac_actrl: RW<GpdacActrl>,
    pub gpdac_bctrl: RW<GpdacBctrl>,
    pub gpdac_data: RW<GpdacData>,
    _reserved3: [u8; 1524],
    pub gpadc_reg_cmd: RW<GpadcRegCmd>,
    pub gpadc_reg_config_1: RW<GpadcRegConfig1>,
    pub gpadc_reg_config_2: RW<GpadcRegConfig2>,
    pub adc_converation_sequence_1: RW<AdcConverationSequence1>,
    pub adc_converation_sequence_2: RW<AdcConverationSequence2>,
    pub adc_converation_sequence_3: RW<AdcConverationSequence3>,
    pub adc_converation_sequence_4: RW<AdcConverationSequence4>,
    pub gpadc_status: RW<GpadcStatus>,
    pub gpadc_interrupt_state: RW<GpadcInterruptState>,
    pub gpadc_result: RW<GpadcResult>,
    pub gpadc_raw_result: RW<GpadcRawResult>,
    pub gpadc_define: RW<GpadcDefine>,
}

/// Generic Analog-to-Digital Converter configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcConfig(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcDmaRdata(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcPirTrain(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcRegCmd(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcRegConfig1(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcRegConfig2(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AdcConverationSequence1(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AdcConverationSequence2(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AdcConverationSequence3(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AdcConverationSequence4(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcStatus(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcInterruptState(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcResult(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcRawResult(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GpadcDefine(u32);

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

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_gpadc_config_functions() {
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
        assert_eq!(offset_of!(RegisterBlock, gpadc_reg_cmd), 0x90C);
        assert_eq!(offset_of!(RegisterBlock, gpadc_reg_config_1), 0x910);
        assert_eq!(offset_of!(RegisterBlock, gpadc_reg_config_2), 0x914);
        assert_eq!(offset_of!(RegisterBlock, adc_converation_sequence_1), 0x918);
        assert_eq!(offset_of!(RegisterBlock, adc_converation_sequence_2), 0x91C);
        assert_eq!(offset_of!(RegisterBlock, adc_converation_sequence_3), 0x920);
        assert_eq!(offset_of!(RegisterBlock, adc_converation_sequence_4), 0x924);
        assert_eq!(offset_of!(RegisterBlock, gpadc_status), 0x928);
        assert_eq!(offset_of!(RegisterBlock, gpadc_interrupt_state), 0x92C);
        assert_eq!(offset_of!(RegisterBlock, gpadc_result), 0x930);
        assert_eq!(offset_of!(RegisterBlock, gpadc_raw_result), 0x934);
        assert_eq!(offset_of!(RegisterBlock, gpadc_define), 0x938);
    }
}
