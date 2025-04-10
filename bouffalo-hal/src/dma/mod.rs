//! Direct Memory Access peripheral.

mod register;

pub use register::{
    BurstSize, ChannelConfig, ChannelRegisters, DmaMode, DmaPeriphReq, EnabledChannels, EndianMode,
    ErrorClear, ErrorState, GlobalConfig, GlobalState, InterruptRegisters, LliControl, LliPool,
    LliTransfer, Periph4Dma01, Periph4Dma2, RawError, RawTransferComplete, RegisterBlock,
    TransferCompleteClear, TransferCompleteState, TransferWidth,
};

use core::ops::Deref;

use crate::glb;

/// DMA peripheral data register address definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaAddr {
    Uart0Tx = 0x2000A000 + 0x88,
    Uart0Rx = 0x2000A000 + 0x8C,
    Uart1Tx = 0x2000A100 + 0x88,
    Uart1Rx = 0x2000A100 + 0x8C,
    Uart2Tx = 0x2000AA00 + 0x88,
    Uart2Rx = 0x2000AA00 + 0x8C,
    Uart3Tx = 0x30002000 + 0x88,
    Uart3Rx = 0x30002000 + 0x8C,
    I2c0Tx = 0x2000A300 + 0x88,
    I2c0Rx = 0x2000A300 + 0x8C,
    I2c1Tx = 0x2000A900 + 0x88,
    I2c1Rx = 0x2000A900 + 0x8C,
    I2c2Tx = 0x30003000 + 0x88,
    I2c2Rx = 0x30003000 + 0x8C,
    I2c3Tx = 0x30004000 + 0x88,
    I2c3Rx = 0x30004000 + 0x8C,
    Spi0Tx = 0x2000A200 + 0x88,
    Spi0Rx = 0x2000A200 + 0x8C,
    Spi1Tx = 0x30008000 + 0x88,
    Spi1Rx = 0x30008000 + 0x8C,
    I2sTx = 0x2000AB00 + 0x88,
    I2sRx = 0x2000AB00 + 0x8C,
    AdcRx = 0x20002000 + 0x04,
    DacTx = 0x20002000 + 0x48,
    IrTx = 0x2000A600 + 0x88,
    WoTx = 0x20000000 + 0xB04,
}

/// Direct Memory Access channel configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaChannelConfig {
    pub direction: DmaMode,
    pub src_req: DmaPeriphReq,
    pub dst_req: DmaPeriphReq,
    pub src_addr_inc: bool,
    pub dst_addr_inc: bool,
    pub src_burst_size: BurstSize,
    pub dst_burst_size: BurstSize,
    pub src_transfer_width: TransferWidth,
    pub dst_transfer_width: TransferWidth,
}

/// Managed Direct Memory Access peripheral.
pub struct Dma<'a, DMA, CH, const D: usize, const C: usize>
where
    DMA: Deref<Target = RegisterBlock>,
    CH: DmaChannel<D, C>,
{
    dma: &'a DMA,
    _channel: CH,
}

impl<'a, DMA: Deref<Target = RegisterBlock>, CH: DmaChannel<D, C>, const D: usize, const C: usize>
    Dma<'a, DMA, CH, D, C>
{
    /// Create a new DMA Peripheral Interface instance.
    #[inline]
    pub fn new(
        dma: &'a DMA,
        _channel: CH,
        channel_config: DmaChannelConfig,
        glb: &glb::v2::RegisterBlock,
    ) -> Self {
        unsafe {
            glb.clock_config_0.modify(|val| val.enable_dma());
            glb.clock_config_1.modify(|val| val.enable_dma::<D>());
            dma.global_config.modify(|val| val.enable_dma());
            dma.channels[C].config.modify(|val| val.disable_ch());
            dma.channels[C].control.modify(|val| {
                let val = if channel_config.src_addr_inc {
                    val.enable_src_addr_inc()
                } else {
                    val.disable_src_addr_inc()
                };
                if channel_config.dst_addr_inc {
                    val.enable_dst_addr_inc()
                } else {
                    val.disable_dst_addr_inc()
                }
            });
            dma.channels[C].control.modify(|val| {
                val.set_src_transfer_width(channel_config.src_transfer_width)
                    .set_dst_transfer_width(channel_config.dst_transfer_width)
                    .set_src_bst_size(channel_config.src_burst_size)
                    .set_dst_bst_size(channel_config.dst_burst_size)
            });
            dma.channels[C]
                .config
                .modify(|val| val.set_dma_mode(channel_config.direction).set_lli_cnt(0));
            dma.channels[C].config.modify(|val| {
                let val = match channel_config.src_req {
                    DmaPeriphReq::Dma01(periph) => val.set_src_periph4dma01(periph),
                    DmaPeriphReq::Dma2(periph) => val.set_src_periph4dma2(periph),
                    DmaPeriphReq::None => val.set_dst_periph4dma01(Periph4Dma01::Uart0Rx), // Just set to 0, not real periph.
                };
                match channel_config.dst_req {
                    DmaPeriphReq::Dma01(periph) => val.set_dst_periph4dma01(periph),
                    DmaPeriphReq::Dma2(periph) => val.set_dst_periph4dma2(periph),
                    DmaPeriphReq::None => val.set_dst_periph4dma01(Periph4Dma01::Uart0Rx), // Just set to 0, not real periph.
                }
            });
            dma.channels[C]
                .config
                .modify(|val| val.enable_cplt_int().enable_err_int());
            dma.channels[C].control.modify(|val| val.disable_cplt_int());
            dma.interrupts
                .transfer_complete_clear
                .write(TransferCompleteClear::default().clear_cplt_int(C as u8));
            dma.interrupts
                .error_clear
                .write(ErrorClear::default().clear_err_int(C as u8));
        }
        Self { dma, _channel }
    }
    /// Configure linked list items.
    #[inline]
    pub fn lli_config(
        &self,
        lli_pool: &mut [LliPool],
        lli_count: u32,
        mut src_addr: u32,
        mut dst_addr: u32,
        transfer_offset: u32,
        last_transfer_len: u32,
    ) {
        let mut ctrl_cfg = self.dma.channels[C].control.read();
        ctrl_cfg = ctrl_cfg.set_transfer_size(4064).disable_cplt_int();

        for i in 0..lli_count {
            lli_pool[i as usize].src_addr = src_addr;
            lli_pool[i as usize].dst_addr = dst_addr;
            lli_pool[i as usize].next_lli = 0;

            if ctrl_cfg.is_src_addr_inc_enabled() {
                src_addr = src_addr + transfer_offset;
            }
            if ctrl_cfg.is_dst_addr_inc_enabled() {
                dst_addr = dst_addr + transfer_offset;
            }
            if i == lli_count - 1 {
                ctrl_cfg = ctrl_cfg
                    .set_transfer_size(last_transfer_len as u16)
                    .enable_cplt_int();
            }
            if i != 0 {
                lli_pool[(i - 1) as usize].next_lli =
                    (&lli_pool[i as usize] as *const LliPool) as u32;
            }

            lli_pool[i as usize].control = ctrl_cfg;
        }
    }
    /// Enable linked list continous mode.
    #[inline]
    pub fn lli_link_head(&self, lli_pool: &mut [LliPool], used_count: usize) {
        lli_pool[used_count - 1].next_lli = (&lli_pool[0] as *const LliPool) as u32;
        unsafe {
            self.dma.channels[C]
                .linked_list_item
                .write(lli_pool[0].next_lli);
        }
    }
    /// Reload linked list items.
    #[inline]
    pub fn lli_reload(
        &self,
        lli_pool: &mut [LliPool],
        max_lli_count: u32,
        transfer: &mut [LliTransfer],
        count: u32,
    ) -> i32 {
        let ctrl_cfg = self.dma.channels[C].control.read();

        let mut lli_count_used_offset = 0;
        let actual_transfer_offset = match ctrl_cfg.src_transfer_width() {
            TransferWidth::Byte => 4064,
            TransferWidth::HalfWord => 4064 << 1,
            TransferWidth::Word => 4064 << 2,
            TransferWidth::DoubleWord => 4064 << 3,
        };

        for i in 0..count {
            let actual_transfer_len = match ctrl_cfg.src_transfer_width() {
                TransferWidth::Byte => transfer[i as usize].nbytes,
                TransferWidth::HalfWord => transfer[i as usize].nbytes >> 1,
                TransferWidth::Word => transfer[i as usize].nbytes >> 2,
                TransferWidth::DoubleWord => transfer[i as usize].nbytes >> 3,
            };

            let mut current_lli_count = actual_transfer_len / 4064 + 1;
            let mut last_transfer_len = actual_transfer_len % 4064;

            if current_lli_count > 1 && last_transfer_len < (4095 - 4064) {
                current_lli_count = current_lli_count - 1;
                last_transfer_len = last_transfer_len + 4064;
            }

            self.lli_config(
                &mut lli_pool[lli_count_used_offset..],
                current_lli_count,
                transfer[i as usize].src_addr,
                transfer[i as usize].dst_addr,
                actual_transfer_offset,
                last_transfer_len,
            );

            if i != 0 {
                lli_pool[lli_count_used_offset - 1].next_lli =
                    (&lli_pool[lli_count_used_offset] as *const LliPool) as u32;
            }

            lli_count_used_offset = lli_count_used_offset + current_lli_count as usize;

            if lli_count_used_offset > max_lli_count as usize {
                // Out of memory.
                return -12;
            }
        }

        unsafe {
            self.dma.channels[C]
                .source_address
                .write(lli_pool[0].src_addr);
            self.dma.channels[C]
                .destination_address
                .write(lli_pool[0].dst_addr);
            self.dma.channels[C]
                .linked_list_item
                .write(lli_pool[0].next_lli);
            self.dma.channels[C].control.write(lli_pool[0].control);
        }
        lli_count_used_offset as i32
    }
    /// Start DMA transfer.
    #[inline]
    pub fn start(&self) {
        unsafe {
            self.dma.channels[C].config.modify(|val| val.enable_ch());
        }
    }
    /// Stop DMA transfer.
    #[inline]
    pub fn stop(&self) {
        unsafe {
            self.dma.channels[C].config.modify(|val| val.disable_ch());
        }
    }
    /// Check if DMA channel is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.dma.channels[C].config.read().is_ch_enabled()
    }
    /// Release DMA channel instance.
    #[inline]
    pub fn free(self) -> (&'a DMA, CH) {
        (self.dma, self._channel)
    }
}

pub trait DmaChannel<const D: usize, const C: usize> {}

// Avaliable DMA channels.
pub struct Dma0Channel0;
pub struct Dma0Channel1;
pub struct Dma0Channel2;
pub struct Dma0Channel3;
pub struct Dma0Channel4;
pub struct Dma0Channel5;
pub struct Dma0Channel6;
pub struct Dma0Channel7;
pub struct Dma1Channel0;
pub struct Dma1Channel1;
pub struct Dma1Channel2;
pub struct Dma1Channel3;
pub struct Dma2Channel0;
pub struct Dma2Channel1;
pub struct Dma2Channel2;
pub struct Dma2Channel3;
pub struct Dma2Channel4;
pub struct Dma2Channel5;
pub struct Dma2Channel6;
pub struct Dma2Channel7;

impl DmaChannel<0, 0> for Dma0Channel0 {}
impl DmaChannel<0, 1> for Dma0Channel1 {}
impl DmaChannel<0, 2> for Dma0Channel2 {}
impl DmaChannel<0, 3> for Dma0Channel3 {}
impl DmaChannel<0, 4> for Dma0Channel4 {}
impl DmaChannel<0, 5> for Dma0Channel5 {}
impl DmaChannel<0, 6> for Dma0Channel6 {}
impl DmaChannel<0, 7> for Dma0Channel7 {}
impl DmaChannel<1, 0> for Dma1Channel0 {}
impl DmaChannel<1, 1> for Dma1Channel1 {}
impl DmaChannel<1, 2> for Dma1Channel2 {}
impl DmaChannel<1, 3> for Dma1Channel3 {}
impl DmaChannel<2, 0> for Dma2Channel0 {}
impl DmaChannel<2, 1> for Dma2Channel1 {}
impl DmaChannel<2, 2> for Dma2Channel2 {}
impl DmaChannel<2, 3> for Dma2Channel3 {}
impl DmaChannel<2, 4> for Dma2Channel4 {}
impl DmaChannel<2, 5> for Dma2Channel5 {}
impl DmaChannel<2, 6> for Dma2Channel6 {}
impl DmaChannel<2, 7> for Dma2Channel7 {}

// Fake dma instance.
pub struct FakeDmaRegisters;

impl Deref for FakeDmaRegisters {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        panic!("FakeDmaRegisters should not be used")
    }
}

// Fake dma channel.
pub struct FakeDmaChannel<const D: usize, const C: usize>;

impl<const D: usize, const C: usize> DmaChannel<D, C> for FakeDmaChannel<D, C> {}
