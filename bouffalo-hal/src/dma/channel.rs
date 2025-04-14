use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::glb;

use super::LliPool;
use super::config::{DmaChannelConfig, Mem2MemChannelConfig, PeripheralId};
use super::register::{
    ErrorClear, LliTransfer, RegisterBlock, TransferCompleteClear, TransferWidth,
};

/// Managed DMA with eight split channels.
pub struct EightChannels<'a, T> {
    /// Channel 0.
    pub ch0: TypedChannel<'a, T>,
    /// Channel 1.
    pub ch1: TypedChannel<'a, T>,
    /// Channel 2.
    pub ch2: TypedChannel<'a, T>,
    /// Channel 3.
    pub ch3: TypedChannel<'a, T>,
    /// Channel 4.
    pub ch4: TypedChannel<'a, T>,
    /// Channel 5.
    pub ch5: TypedChannel<'a, T>,
    /// Channel 6.
    pub ch6: TypedChannel<'a, T>,
    /// Channel 7.
    pub ch7: TypedChannel<'a, T>,
}

/// Managed DMA with four split channels.
pub struct FourChannels<'a, T> {
    /// Channel 0.
    pub ch0: TypedChannel<'a, T>,
    /// Channel 1.
    pub ch1: TypedChannel<'a, T>,
    /// Channel 2.
    pub ch2: TypedChannel<'a, T>,
    /// Channel 3.
    pub ch3: TypedChannel<'a, T>,
}

/// Channel with dedicated peripheral type.
pub struct TypedChannel<'a, T> {
    inner: UntypedChannel<'a>,
    _type_of_peripheral: PhantomData<T>,
}

impl<'a, T: PeripheralId> TypedChannel<'a, T> {
    pub fn configure(&mut self, channel_config: DmaChannelConfig<T>) {
        let dma = self.inner.dma;
        let id = self.inner.channel_id;
        unsafe {
            dma.channels[id].config.modify(|val| val.disable_ch());
            dma.channels[id].control.modify(|val| {
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
            dma.channels[id].control.modify(|val| {
                val.set_src_transfer_width(channel_config.src_transfer_width)
                    .set_dst_transfer_width(channel_config.dst_transfer_width)
                    .set_src_bst_size(channel_config.src_burst_size)
                    .set_dst_bst_size(channel_config.dst_burst_size)
            });
            dma.channels[id]
                .config
                .modify(|val| val.set_dma_mode(channel_config.direction).set_lli_cnt(0));
            dma.channels[id].config.modify(|val| {
                let val = match channel_config.src_req {
                    Some(periph) => val.set_src_periph(periph),
                    None => val.clear_src_periph(),
                };
                match channel_config.dst_req {
                    Some(periph) => val.set_dst_periph(periph),
                    None => val.clear_dst_periph(),
                }
            });
            dma.channels[id]
                .config
                .modify(|val| val.enable_cplt_int().enable_err_int());
            dma.channels[id]
                .control
                .modify(|val| val.disable_cplt_int());
            dma.interrupts
                .transfer_complete_clear
                .write(TransferCompleteClear::default().clear_cplt_int(id as u8));
            dma.interrupts
                .error_clear
                .write(ErrorClear::default().clear_err_int(id as u8));
        }
    }
}

impl<'a, T> Deref for TypedChannel<'a, T> {
    type Target = UntypedChannel<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a, T> DerefMut for TypedChannel<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Channel without a dedicated peripheral type.
pub struct UntypedChannel<'a> {
    dma: &'a RegisterBlock,
    channel_id: usize,
}

impl<'a> UntypedChannel<'a> {
    pub fn memory_to_memory(&mut self, channel_config: Mem2MemChannelConfig) {
        let dma = self.dma;
        let id = self.channel_id;
        unsafe {
            dma.channels[id].config.modify(|val| val.disable_ch());
            dma.channels[id].control.modify(|val| {
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
            dma.channels[id].control.modify(|val| {
                val.set_src_transfer_width(channel_config.src_transfer_width)
                    .set_dst_transfer_width(channel_config.dst_transfer_width)
                    .set_src_bst_size(channel_config.src_burst_size)
                    .set_dst_bst_size(channel_config.dst_burst_size)
            });
            dma.channels[id]
                .config
                .modify(|val| val.set_dma_mode(channel_config.direction).set_lli_cnt(0));
            dma.channels[id]
                .config
                .modify(|val| val.clear_src_periph().clear_dst_periph());
            dma.channels[id]
                .config
                .modify(|val| val.enable_cplt_int().enable_err_int());
            dma.channels[id]
                .control
                .modify(|val| val.disable_cplt_int());
            dma.interrupts
                .transfer_complete_clear
                .write(TransferCompleteClear::default().clear_cplt_int(id as u8));
            dma.interrupts
                .error_clear
                .write(ErrorClear::default().clear_err_int(id as u8));
        }
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
        let mut ctrl_cfg = self.dma.channels[self.channel_id].control.read();
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
            self.dma.channels[self.channel_id]
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
        let ctrl_cfg = self.dma.channels[self.channel_id].control.read();

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
            self.dma.channels[self.channel_id]
                .source_address
                .write(lli_pool[0].src_addr);
            self.dma.channels[self.channel_id]
                .destination_address
                .write(lli_pool[0].dst_addr);
            self.dma.channels[self.channel_id]
                .linked_list_item
                .write(lli_pool[0].next_lli);
            self.dma.channels[self.channel_id]
                .control
                .write(lli_pool[0].control);
        }
        lli_count_used_offset as i32
    }
    /// Start DMA transfer.
    #[inline]
    pub fn start(&self) {
        unsafe {
            self.dma.channels[self.channel_id]
                .config
                .modify(|val| val.enable_ch());
        }
    }
    /// Stop DMA transfer.
    #[inline]
    pub fn stop(&self) {
        unsafe {
            self.dma.channels[self.channel_id]
                .config
                .modify(|val| val.disable_ch());
        }
    }
    /// Check if DMA channel is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.dma.channels[self.channel_id]
            .config
            .read()
            .is_ch_enabled()
    }
}

impl<'a, T> EightChannels<'a, T> {
    #[doc(hidden)]
    pub fn __new<const D: usize>(dma: &'a RegisterBlock, glb: &glb::v2::RegisterBlock) -> Self {
        unsafe {
            glb.clock_config_0.modify(|val| val.enable_dma());
            glb.clock_config_1.modify(|val| val.enable_dma::<D>());
            dma.global_config.modify(|val| val.enable_dma());
        }
        Self {
            ch0: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 0 },
                _type_of_peripheral: PhantomData,
            },
            ch1: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 1 },
                _type_of_peripheral: PhantomData,
            },
            ch2: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 2 },
                _type_of_peripheral: PhantomData,
            },
            ch3: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 3 },
                _type_of_peripheral: PhantomData,
            },
            ch4: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 4 },
                _type_of_peripheral: PhantomData,
            },
            ch5: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 5 },
                _type_of_peripheral: PhantomData,
            },
            ch6: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 6 },
                _type_of_peripheral: PhantomData,
            },
            ch7: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 7 },
                _type_of_peripheral: PhantomData,
            },
        }
    }
}

impl<'a, T> FourChannels<'a, T> {
    #[doc(hidden)]
    pub fn __new<const D: usize>(dma: &'a RegisterBlock, glb: &glb::v2::RegisterBlock) -> Self {
        unsafe {
            glb.clock_config_0.modify(|val| val.enable_dma());
            glb.clock_config_1.modify(|val| val.enable_dma::<D>());
            dma.global_config.modify(|val| val.enable_dma());
        }
        Self {
            ch0: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 0 },
                _type_of_peripheral: PhantomData,
            },
            ch1: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 1 },
                _type_of_peripheral: PhantomData,
            },
            ch2: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 2 },
                _type_of_peripheral: PhantomData,
            },
            ch3: TypedChannel {
                inner: UntypedChannel { dma, channel_id: 3 },
                _type_of_peripheral: PhantomData,
            },
        }
    }
}
