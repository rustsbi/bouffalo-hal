use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::glb;

use super::config::{DmaChannelConfig, Mem2MemChannelConfig, PeripheralId};
use super::register::{
    ErrorClear, LliTransfer, RegisterBlock, TransferCompleteClear, TransferWidth,
};
use super::{Instance, LliPool};

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

impl<'a, T> TypedChannel<'a, T> {
    /// Internal constructor.
    #[inline]
    const fn new(dma: &'a RegisterBlock, channel_id: usize) -> Self {
        Self {
            inner: UntypedChannel { dma, channel_id },
            _type_of_peripheral: PhantomData,
        }
    }
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
            if i > 0 {
                lli_pool[(i - 1) as usize].next_lli =
                    (&lli_pool[i as usize] as *const LliPool) as u32;
            }

            lli_pool[i as usize].control = ctrl_cfg;
        }

        unsafe {
            l1c_dcache_clean_range(
                lli_pool.as_ptr() as usize,
                lli_pool.len() * core::mem::size_of::<LliPool>(),
            );
        }
    }

    /// Enable linked list continuous mode.
    #[inline]
    pub fn lli_link_head(&self, lli_pool: &mut [LliPool], used_count: usize) {
        lli_pool[used_count - 1].next_lli = (&lli_pool[0] as *const LliPool) as u32;

        unsafe {
            l1c_dcache_clean_range(
                &lli_pool[used_count - 1] as *const _ as usize,
                core::mem::size_of::<LliPool>(),
            );
        }

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
            TransferWidth::HalfWord => 4064 * 2,
            TransferWidth::Word => 4064 * 4,
            TransferWidth::DoubleWord => 4064 * 8,
        };

        for i in 0..count {
            let actual_transfer_len = match ctrl_cfg.src_transfer_width() {
                TransferWidth::Byte => transfer[i as usize].nbytes,
                TransferWidth::HalfWord => {
                    if transfer[i as usize].nbytes % 2 != 0 {
                        return -1;
                    }
                    transfer[i as usize].nbytes / 2
                }
                TransferWidth::Word => {
                    if transfer[i as usize].nbytes % 4 != 0 {
                        return -1;
                    }
                    transfer[i as usize].nbytes / 4
                }
                TransferWidth::DoubleWord => {
                    if transfer[i as usize].nbytes % 8 != 0 {
                        return -1;
                    }
                    transfer[i as usize].nbytes / 8
                }
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

            if i > 0 && lli_count_used_offset > 0 {
                lli_pool[lli_count_used_offset - 1].next_lli =
                    (&lli_pool[lli_count_used_offset] as *const LliPool) as u32;
            }

            lli_count_used_offset = lli_count_used_offset + current_lli_count as usize;

            if lli_count_used_offset > max_lli_count as usize {
                // Out of memory.
                return -12;
            }
        }

        if lli_count_used_offset > 0 {
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

            unsafe {
                l1c_dcache_clean_range(
                    lli_pool.as_ptr() as usize,
                    lli_count_used_offset * core::mem::size_of::<LliPool>(),
                );
            }
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
    /// Create an eight-channel DMA structure.
    pub fn new<const D: usize>(dma: impl Instance<'a>, glb: &glb::v2::RegisterBlock) -> Self {
        let dma = dma.register_block();
        unsafe {
            glb.clock_config_0.modify(|val| val.enable_dma());
            glb.clock_config_1.modify(|val| val.enable_dma::<D>());
            dma.global_config.modify(|val| val.enable_dma());
        }
        Self {
            ch0: TypedChannel::new(dma, 0),
            ch1: TypedChannel::new(dma, 1),
            ch2: TypedChannel::new(dma, 2),
            ch3: TypedChannel::new(dma, 3),
            ch4: TypedChannel::new(dma, 4),
            ch5: TypedChannel::new(dma, 5),
            ch6: TypedChannel::new(dma, 6),
            ch7: TypedChannel::new(dma, 7),
        }
    }
}

impl<'a, T> FourChannels<'a, T> {
    /// Create an four-channel DMA structure.
    pub fn new<const D: usize>(dma: impl Instance<'a>, glb: &glb::v2::RegisterBlock) -> Self {
        let dma = dma.register_block();
        unsafe {
            glb.clock_config_0.modify(|val| val.enable_dma());
            glb.clock_config_1.modify(|val| val.enable_dma::<D>());
            dma.global_config.modify(|val| val.enable_dma());
        }
        Self {
            ch0: TypedChannel::new(dma, 0),
            ch1: TypedChannel::new(dma, 1),
            ch2: TypedChannel::new(dma, 2),
            ch3: TypedChannel::new(dma, 3),
        }
    }
}
/// Clean (write back) L1 data cache for a specific memory range.
///
/// # Parameters
/// - `addr`: Start address of the memory range to clean
/// - `len`: Length in bytes of the memory range to clean
///
/// # Safety
/// This function uses raw assembly instructions and directly manipulates cache hardware.
/// The caller must ensure the addresses are valid memory locations.
#[inline]
pub unsafe fn l1c_dcache_clean_range(addr: usize, len: usize) {
    // First check if the address is valid for cache operations
    if !check_cache_addr(addr) {
        return;
    }

    // Cache line size is 32 bytes for Bouffalo chips
    const CACHE_LINE_SIZE: usize = 32;

    // Align start address to cache line boundary (round down)
    let start = addr & !(CACHE_LINE_SIZE - 1);

    // Calculate end address and align to cache line boundary (round up)
    let end = (addr + len + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);

    let mut current = start;

    while current < end {
        // Use RISC-V custom instruction to clean cache line
        unsafe {
            core::arch::asm!(
                "mv x10, {0}",      // Move value to x10 register (same as a0)
                ".word 0x02A5F02F", // dcache.cpa instruction (cache push address)
                in(reg) current,    // Input in any register
                options(nostack)
            );
        }

        current += CACHE_LINE_SIZE;
    }

    // Memory fence to ensure all cache operations are complete
    unsafe {
        core::arch::asm!("fence", options(nostack));
    }
}

/// Check if address is valid for cache operations
///
/// # Returns
/// `true` if address is valid for cache operations, `false` otherwise
#[inline]
fn check_cache_addr(addr: usize) -> bool {
    // For Bouffalo chips, typically addresses in certain ranges are cacheable
    // This is a simplified implementation - actual implementation should check
    // against the specific chip's memory map

    // Example: Check if address is in DTCM or main RAM regions
    // Adjust these values based on the specific Bouffalo chip you're targeting
    const DTCM_START: usize = 0x2000_0000;
    const DTCM_END: usize = 0x2003_FFFF;
    const RAM_START: usize = 0x6000_0000;
    const RAM_END: usize = 0x60FF_FFFF;

    (addr >= DTCM_START && addr <= DTCM_END) || (addr >= RAM_START && addr <= RAM_END)
}
