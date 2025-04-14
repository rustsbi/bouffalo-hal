//! BL808 tri-core heterogeneous Wi-Fi 802.11b/g/n, Bluetooth 5, Zigbee AIoT system-on-chip.

use crate::{HalBasicConfig, HalFlashConfig, HalPatchCfg};

#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[naked]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
    unsafe {
        use crate::arch::rvi::Stack;
        const LEN_STACK_MCU: usize = 1 * 1024;
        #[unsafe(link_section = ".bss.uninit")]
        static mut STACK: Stack<LEN_STACK_MCU> = Stack([0; LEN_STACK_MCU]);
        core::arch::naked_asm!(
            "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
            "   la      t1, sbss
            la      t2, ebss
        1:  bgeu    t1, t2, 1f
            sw      zero, 0(t1)
            addi    t1, t1, 4
            j       1b
        1:",
            "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            lw      t6, 0(t3)
            sw      t6, 0(t4)
            addi    t3, t3, 4
            addi    t4, t4, 4
            j       1b
        1:",
            "   la      t0, {trap_entry}
            ori     t0, t0, {trap_mode}
            csrw    mtvec, t0",
            "   li      t1, {stack_protect_pmp_address_begin}
            csrw    pmpaddr0, t1
            li      t1, {stack_protect_pmp_address_end}
            csrw    pmpaddr1, t1
            li      t2, {stack_protect_pmp_flags}
            csrw    pmpcfg0, t2",
            "   call  {main}",
            stack = sym STACK,
            hart_stack_size = const LEN_STACK_MCU,
            trap_entry = sym trap_vectored,
            trap_mode = const 1, // RISC-V standard vectored trap
            // Set PMP entry to block U/S-mode stack access (TOR, no R/W/X permissions)
            stack_protect_pmp_address_begin = const {0x62030000 >> 2},
            stack_protect_pmp_address_end = const {(0x62030000 + 160 * 1024) >> 2},
            stack_protect_pmp_flags = const 0b00001000 << 8,
            main = sym main,
        )
    }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[naked]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
    unsafe {
        use crate::arch::rvi::Stack;
        const LEN_STACK_DSP: usize = 4 * 1024;
        #[unsafe(link_section = ".bss.uninit")]
        static mut STACK: Stack<LEN_STACK_DSP> = Stack([0; LEN_STACK_DSP]);
        core::arch::naked_asm!(
            "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
            "   la      t1, sbss
            la      t2, ebss
        1:  bgeu    t1, t2, 1f
            sd      zero, 0(t1)
            addi    t1, t1, 8 
            j       1b
        1:",
            "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            ld      t6, 0(t3)
            sd      t6, 0(t4)
            addi    t3, t3, 8
            addi    t4, t4, 8
            j       1b
        1:",
            "   la      t0, {trap_entry}
            ori     t0, t0, {trap_mode}
            csrw    mtvec, t0",
            "   li      t1, {stack_protect_pmp_address_begin}
            csrw    pmpaddr0, t1
            li      t1, {stack_protect_pmp_address_end}
            csrw    pmpaddr1, t1
            li      t2, {stack_protect_pmp_flags}
            csrw    pmpcfg0, t2",
            "   call    {main}",
            stack = sym STACK,
            hart_stack_size = const LEN_STACK_DSP,
            trap_entry = sym trap_vectored,
            trap_mode = const 1, // RISC-V standard vectored trap
            // Set PMP entry to block U/S-mode stack access (TOR, no R/W/X permissions)
            stack_protect_pmp_address_begin = const {0x3F000000 >> 2},
            stack_protect_pmp_address_end = const {(0x3F000000 + 32 * 1024) >> 2},
            stack_protect_pmp_flags = const 0b00001000 << 8,
            main = sym main,
        )
    }
}

#[cfg(all(feature = "bl808-lp", target_arch = "riscv32"))]
#[naked]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
    unsafe {
        use crate::arch::rve::Stack;
        const LEN_STACK_LP: usize = 1 * 1024;
        #[unsafe(link_section = ".bss.uninit")]
        static mut STACK: Stack<LEN_STACK_LP> = Stack([0; LEN_STACK_LP]);
        core::arch::naked_asm!(
            "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
            "   la      t1, sbss
            la      t2, ebss
        1:  bgeu    t1, t2, 1f
            sw      zero, 0(t1)
            addi    t1, t1, 4
            j       1b
        1:",
            "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            lw      t6, 0(t3)
            sw      t6, 0(t4)
            addi    t3, t3, 4
            addi    t4, t4, 4
            j       1b
        1:",
            // TODO trap support
            // TODO pmp support
            "   call  {main}",
            stack = sym STACK,
            hart_stack_size = const LEN_STACK_LP,
            main = sym main,
        )
    }
}

#[cfg(any(
    all(feature = "bl808-mcu", target_arch = "riscv32"),
    all(feature = "bl808-lp", target_arch = "riscv32"),
    all(feature = "bl808-dsp", target_arch = "riscv64")
))]
unsafe extern "Rust" {
    // This symbol is generated by `#[entry]` macro
    fn main() -> !;
}

// Alignment of this function is ensured by `build.rs` script.
#[cfg(any(
    all(feature = "bl808-mcu", target_arch = "riscv32"),
    all(feature = "bl808-dsp", target_arch = "riscv64")
))]
#[unsafe(link_section = ".trap.trap-entry")]
#[naked]
unsafe extern "C" fn trap_vectored() -> ! {
    unsafe {
        core::arch::naked_asm!(
            ".p2align 2",
            "j {exceptions}",
            "j {supervisor_software}",
            "j {reserved}",
            "j {machine_software}",
            "j {reserved}",
            "j {supervisor_timer}",
            "j {reserved}",
            "j {machine_timer}",
            "j {reserved}",
            "j {supervisor_external}",
            "j {reserved}",
            "j {machine_external}",
            "j {reserved}",
            "j {reserved}",
            "j {reserved}",
            "j {reserved}",
            "j {reserved}",
            "j {thead_hpm_overflow}",
            exceptions = sym exceptions_trampoline,
            supervisor_software = sym reserved,
            machine_software = sym reserved,
            supervisor_timer = sym reserved,
            machine_timer = sym reserved,
            machine_external = sym machine_external_trampoline,
            supervisor_external = sym reserved,
            thead_hpm_overflow = sym reserved,
            reserved = sym reserved,
        )
    }
}

#[cfg(any(
    all(feature = "bl808-mcu", target_arch = "riscv32"),
    all(feature = "bl808-dsp", target_arch = "riscv64")
))]
#[naked]
unsafe extern "C" fn reserved() -> ! {
    unsafe { core::arch::naked_asm!("1: j   1b") }
}

#[cfg(any(all(feature = "bl808-dsp", target_arch = "riscv64")))]
unsafe extern "C" {
    fn exceptions(tf: &mut crate::arch::rvi::TrapFrame);
}

// TODO exceptions_trampoline for bl808-mcu
#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[naked]
unsafe extern "C" fn exceptions_trampoline() -> ! {
    unsafe { core::arch::naked_asm!("") }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[naked]
unsafe extern "C" fn exceptions_trampoline() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "addi   sp, sp, -19*8",
            "sd     ra, 0*8(sp)",
            "sd     t0, 1*8(sp)",
            "sd     t1, 2*8(sp)",
            "sd     t2, 3*8(sp)",
            "sd     a0, 4*8(sp)",
            "sd     a1, 5*8(sp)",
            "sd     a2, 6*8(sp)",
            "sd     a3, 7*8(sp)",
            "sd     a4, 8*8(sp)",
            "sd     a5, 9*8(sp)",
            "sd     a6, 10*8(sp)",
            "sd     a7, 11*8(sp)",
            "sd     t3, 12*8(sp)",
            "sd     t4, 13*8(sp)",
            "sd     t5, 14*8(sp)",
            "sd     t6, 15*8(sp)",
            "csrr   t0, mcause",
            "sd     t0, 16*8(sp)",
            "csrr   t1, mepc",
            "sd     t1, 17*8(sp)",
            "csrr   t2, mstatus",
            "sd     t2, 18*8(sp)",
            // "csrs   mstatus, 8", // TODO: disallow nested interrupt by now
            "mv     a0, sp",
            "call   {rust_exceptions}",
            "ld     t0, 16*8(sp)",
            "csrw   mcause, t0",
            "ld     t1, 17*8(sp)",
            "csrw   mepc, t1",
            "ld     t2, 18*8(sp)",
            "csrw   mstatus, t2",
            "ld     ra, 0*8(sp)",
            "ld     t0, 1*8(sp)",
            "ld     t1, 2*8(sp)",
            "ld     t2, 3*8(sp)",
            "ld     a0, 4*8(sp)",
            "ld     a1, 5*8(sp)",
            "ld     a2, 6*8(sp)",
            "ld     a3, 7*8(sp)",
            "ld     a4, 8*8(sp)",
            "ld     a5, 9*8(sp)",
            "ld     a6, 10*8(sp)",
            "ld     a7, 11*8(sp)",
            "ld     t3, 12*8(sp)",
            "ld     t4, 13*8(sp)",
            "ld     t5, 14*8(sp)",
            "ld     t6, 15*8(sp)",
            "addi   sp, sp, 19*8",
            "mret",
            rust_exceptions = sym exceptions,
        )
    }
}

// TODO machine_external_trampoline for bl808-mcu
#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[naked]
unsafe extern "C" fn machine_external_trampoline() -> ! {
    unsafe { core::arch::naked_asm!("") }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[naked]
unsafe extern "C" fn machine_external_trampoline() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "addi   sp, sp, -19*8",
            "sd     ra, 0*8(sp)",
            "sd     t0, 1*8(sp)",
            "sd     t1, 2*8(sp)",
            "sd     t2, 3*8(sp)",
            "sd     a0, 4*8(sp)",
            "sd     a1, 5*8(sp)",
            "sd     a2, 6*8(sp)",
            "sd     a3, 7*8(sp)",
            "sd     a4, 8*8(sp)",
            "sd     a5, 9*8(sp)",
            "sd     a6, 10*8(sp)",
            "sd     a7, 11*8(sp)",
            "sd     t3, 12*8(sp)",
            "sd     t4, 13*8(sp)",
            "sd     t5, 14*8(sp)",
            "sd     t6, 15*8(sp)",
            "csrr   t0, mcause",
            "sd     t0, 16*8(sp)",
            "csrr   t1, mepc",
            "sd     t1, 17*8(sp)",
            "csrr   t2, mstatus",
            "sd     t2, 18*8(sp)",
            // "csrs   mstatus, 8", // TODO: disallow nested interrupt by now
            "mv     a0, sp",
            "call   {rust_all_traps}",
            "ld     t0, 16*8(sp)",
            "csrw   mcause, t0",
            "ld     t1, 17*8(sp)",
            "csrw   mepc, t1",
            "ld     t2, 18*8(sp)",
            "csrw   mstatus, t2",
            "ld     ra, 0*8(sp)",
            "ld     t0, 1*8(sp)",
            "ld     t1, 2*8(sp)",
            "ld     t2, 3*8(sp)",
            "ld     a0, 4*8(sp)",
            "ld     a1, 5*8(sp)",
            "ld     a2, 6*8(sp)",
            "ld     a3, 7*8(sp)",
            "ld     a4, 8*8(sp)",
            "ld     a5, 9*8(sp)",
            "ld     a6, 10*8(sp)",
            "ld     a7, 11*8(sp)",
            "ld     t3, 12*8(sp)",
            "ld     t4, 13*8(sp)",
            "ld     t5, 14*8(sp)",
            "ld     t6, 15*8(sp)",
            "addi   sp, sp, 19*8",
            "mret",
            rust_all_traps = sym rust_bl808_dsp_machine_external,
        )
    }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
fn rust_bl808_dsp_machine_external(_tf: &mut crate::arch::rvi::TrapFrame) {
    let plic: PLIC = unsafe { core::mem::transmute(()) };
    if let Some(source) = plic.claim(D0Machine) {
        let idx = source.get() as usize;
        if idx >= 16 && idx < 16 + 67 {
            unsafe { (D0_INTERRUPT_HANDLERS[idx - 16])() };
        }
        plic.complete(D0Machine, RawPlicSource(source));
    }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
static D0_INTERRUPT_HANDLERS: [unsafe extern "C" fn(); 67] = [
    bmx_dsp_bus_err,
    dsp_reserved1,
    dsp_reserved2,
    dsp_reserved3,
    uart3,
    i2c2,
    i2c3,
    spi1,
    dsp_reserved4,
    dsp_reserved5,
    seof_int0,
    seof_int1,
    seof_int2,
    dvp2_bus_int0,
    dvp2_bus_int1,
    dvp2_bus_int2,
    dvp2_bus_int3,
    h264_bs,
    h264_frame,
    h264_seq_done,
    mjpeg,
    h264_s_bs,
    h264_s_frame,
    h264_s_seq_done,
    dma2_int0,
    dma2_int1,
    dma2_int2,
    dma2_int3,
    dma2_int4,
    dma2_int5,
    dma2_int6,
    dma2_int7,
    dsp_reserved6,
    dsp_reserved7,
    dsp_reserved8,
    dsp_reserved9,
    dsp_reserved10,
    mipi_csi,
    ipc_d0,
    dsp_reserved11,
    mjdec,
    dvp2_bus_int4,
    dvp2_bus_int5,
    dvp2_bus_int6,
    dvp2_bus_int7,
    dma2_d_int0,
    dma2_d_int1,
    display,
    pwm,
    seof_int3,
    dsp_reserved12,
    dsp_reserved13,
    osd,
    dbi,
    dsp_reserved14,
    osda_bus_drain,
    osdb_bus_drain,
    osd_pb,
    dsp_reserved15,
    mipi_dsi,
    dsp_reserved16,
    timer0,
    timer1,
    wdt,
    audio,
    wl_all,
    pds,
];

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
unsafe extern "C" {
    fn bmx_dsp_bus_err();
    fn dsp_reserved1();
    fn dsp_reserved2();
    fn dsp_reserved3();
    fn uart3();
    fn i2c2();
    fn i2c3();
    fn spi1();
    fn dsp_reserved4();
    fn dsp_reserved5();
    fn seof_int0();
    fn seof_int1();
    fn seof_int2();
    fn dvp2_bus_int0();
    fn dvp2_bus_int1();
    fn dvp2_bus_int2();
    fn dvp2_bus_int3();
    fn h264_bs();
    fn h264_frame();
    fn h264_seq_done();
    fn mjpeg();
    fn h264_s_bs();
    fn h264_s_frame();
    fn h264_s_seq_done();
    fn dma2_int0();
    fn dma2_int1();
    fn dma2_int2();
    fn dma2_int3();
    fn dma2_int4();
    fn dma2_int5();
    fn dma2_int6();
    fn dma2_int7();
    fn dsp_reserved6();
    fn dsp_reserved7();
    fn dsp_reserved8();
    fn dsp_reserved9();
    fn dsp_reserved10();
    fn mipi_csi();
    fn ipc_d0();
    fn dsp_reserved11();
    fn mjdec();
    fn dvp2_bus_int4();
    fn dvp2_bus_int5();
    fn dvp2_bus_int6();
    fn dvp2_bus_int7();
    fn dma2_d_int0();
    fn dma2_d_int1();
    fn display();
    fn pwm();
    fn seof_int3();
    fn dsp_reserved12();
    fn dsp_reserved13();
    fn osd();
    fn dbi();
    fn dsp_reserved14();
    fn osda_bus_drain();
    fn osdb_bus_drain();
    fn osd_pb();
    fn dsp_reserved15();
    fn mipi_dsi();
    fn dsp_reserved16();
    fn timer0();
    fn timer1();
    fn wdt();
    fn audio();
    fn wl_all();
    fn pds();
}

/// D0 core machine mode context.
pub struct D0Machine;

impl plic::HartContext for D0Machine {
    #[inline]
    fn index(self) -> usize {
        0
    }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
struct RawPlicSource(core::num::NonZeroU32);

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
impl plic::InterruptSource for RawPlicSource {
    #[inline]
    fn id(self) -> core::num::NonZeroU32 {
        self.0
    }
}

/// DSP core PLIC interrupt source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DspInterrupt {
    /// UART3 interrupt.
    Uart3 = 16 + 4,
    /// I2C2 interrupt.
    I2c2 = 16 + 5,
    /// I2C3 interrupt.
    I2c3 = 16 + 6,
    /// SPI1 interrupt.
    Spi1 = 16 + 7,
    /// DMA2 interrupt 0.
    Dma2Int0 = 16 + 24,
    /// DMA2 interrupt 1.
    Dma2Int1 = 16 + 25,
    /// DMA2 interrupt 2.
    Dma2Int2 = 16 + 26,
    /// DMA2 interrupt 3.
    Dma2Int3 = 16 + 27,
    /// DMA2 interrupt 4.
    Dma2Int4 = 16 + 28,
    /// DMA2 interrupt 5.
    Dma2Int5 = 16 + 29,
    /// DMA2 interrupt 6.
    Dma2Int6 = 16 + 30,
    /// DMA2 interrupt 7.
    Dma2Int7 = 16 + 31,
    /// EMAC2 interrupt.
    Emac2 = 16 + 36,
    /// DMA2D interrupt 0.
    Dma2dInt0 = 16 + 45,
    /// DMA2D interrupt 1.
    Dma2dInt1 = 16 + 46,
    /// PWM interrupt.
    Pwm = 16 + 48,
    /// TIM1 channel 0 interrupt.
    Tim1Ch0 = 16 + 61,
    /// TIM1 interrupt.
    Tim1Ch1 = 16 + 62,
    /// TIM1 WDT interrupt.
    Tim1Wdt = 16 + 63,
    /// AUDIO interrupt.
    Audio = 16 + 64,
    /// PDS interrupt.
    Pds = 16 + 66,
}

impl plic::InterruptSource for DspInterrupt {
    #[inline]
    fn id(self) -> core::num::NonZeroU32 {
        core::num::NonZeroU32::new(self as u32).unwrap()
    }
}

/// MCU and Low-Power core interrupt source.
pub enum McuLpInterrupt {
    /// DMA0 all interrupt.
    Dma0All = 16 + 15,
    /// DMA1 all interrupt.
    Dma1All = 16 + 16,
    /// IR transmit interrupt.
    IrTx = 16 + 19,
    /// IR receive interrupt.
    IrRx = 16 + 20,
    /// USB interrupt.
    Usb = 16 + 21,
    /// EMAC interrupt.
    Emac = 16 + 24,
    /// GPADC DMA interrupt.
    GpadcDma = 16 + 25,
    /// SPI0 interrupt.
    Spi0 = 16 + 27,
    /// UART0 interrupt.
    Uart0 = 16 + 28,
    /// UART1 interrupt.
    Uart1 = 16 + 29,
    /// UART2 interrupt.
    Uart2 = 16 + 30,
    /// GPIO DMA interrupt.
    GpioDma = 16 + 31,
    /// I2C0 interrupt.
    I2c0 = 16 + 32,
    /// I2C1 interrupt.
    I2c1 = 16 + 39,
    /// PWM interrupt.
    Pwm = 16 + 33,
    /// TIM0 channel 0 interrupt.
    Tim0Ch0 = 16 + 36,
    /// TIM0 channel 1 interrupt.
    Tim0Ch1 = 16 + 37,
    /// TIM0 WDT interrupt.
    Tim0Wdt = 16 + 38,
    /// I2S interrupt.
    I2s = 16 + 40,
    /// GPIO interrupt.
    Gpio = 16 + 44,
    /// PDS wakeup interrupt.
    PdsWakeup = 16 + 50,
    /// HBN out 0 interrupt.
    HbnOut0 = 16 + 51,
    /// HBN out 1 interrupt.
    HbnOut1 = 16 + 52,
}

/// Clock configuration at boot-time.
#[cfg(any(doc, feature = "bl808-mcu", feature = "bl808-dsp"))]
#[unsafe(link_section = ".head.clock")]
pub static CLOCK_CONFIG: HalPllConfig = HalPllConfig::new(HalSysClkConfig {
    xtal_type: 0x07,
    mcu_clk: 0x04,
    mcu_clk_div: 0x00,
    mcu_bclk_div: 0x00,

    mcu_pbclk_div: 0x03,
    lp_div: 0x01,
    dsp_clk: 0x03,
    dsp_clk_div: 0x00,

    dsp_bclk_div: 0x01,
    dsp_pbclk: 0x02,
    dsp_pbclk_div: 0x00,
    emi_clk: 0x02,

    emi_clk_div: 0x01,
    flash_clk_type: 0x01,
    flash_clk_div: 0x00,
    wifipll_pu: 0x01,

    aupll_pu: 0x01,
    cpupll_pu: 0x01,
    mipipll_pu: 0x01,
    uhspll_pu: 0x01,
});

/// Miscellaneous image flags.
#[cfg(any(doc, feature = "bl808-mcu", feature = "bl808-dsp"))]
#[unsafe(link_section = ".head.base.flag")]
pub static BASIC_CONFIG_FLAGS: u32 = 0x654c0100;

/// Processor core configuration.
#[cfg(any(doc, feature = "bl808-mcu", feature = "bl808-dsp"))]
#[unsafe(link_section = ".head.cpu")]
pub static CPU_CONFIG: [HalCpuCfg; 3] = [
    #[cfg(feature = "bl808-mcu")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0x58000000,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-mcu"))]
    HalCpuCfg::disabled(),
    #[cfg(feature = "bl808-dsp")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0x58000000,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-dsp"))]
    HalCpuCfg::disabled(),
    #[cfg(feature = "bl808-lp")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-lp"))]
    HalCpuCfg {
        config_enable: 0,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 1476722688,
        cache_range_l: 1476657152,
        image_address_offset: 0x42000,
        boot_entry: 0x58040000,
        msp_val: 0,
    },
];

/// Code patches on flash reading.
#[cfg(any(doc, feature = "bl808-mcu", feature = "bl808-dsp"))]
#[unsafe(link_section = ".head.patch.on-read")]
pub static PATCH_ON_READ: [HalPatchCfg; 4] = [
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
];

/// Code patches on jump and run stage.
#[cfg(any(doc, feature = "bl808-mcu", feature = "bl808-dsp"))]
#[unsafe(link_section = ".head.patch.on-jump")]
pub static PATCH_ON_JUMP: [HalPatchCfg; 4] = [
    HalPatchCfg {
        addr: 0x20000320,
        value: 0x0,
    },
    HalPatchCfg {
        addr: 0x2000F038,
        value: 0x18000000,
    },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
];

/// Full ROM bootloading header.
#[repr(C)]
pub struct HalBootheader {
    magic: u32,
    revision: u32,
    flash_cfg: HalFlashConfig,
    clk_cfg: HalPllConfig,
    basic_cfg: HalBasicConfig,
    cpu_cfg: [HalCpuCfg; 3],
    /// Address of partition table 0.
    boot2_pt_table_0: u32,
    /// Address of partition table 1.
    boot2_pt_table_1: u32,
    /// Address of flashcfg table list.
    flash_cfg_table_addr: u32,
    /// Flashcfg table list len.
    flash_cfg_table_len: u32,
    /// Do patch when read flash.
    patch_on_read: [HalPatchCfg; 4],
    /// Do patch when jump.
    patch_on_jump: [HalPatchCfg; 4],
    _reserved: [u32; 5],
    crc32: u32,
}

/// Hardware system clock configuration.
#[repr(C)]
pub struct HalSysClkConfig {
    xtal_type: u8,
    mcu_clk: u8,
    mcu_clk_div: u8,
    mcu_bclk_div: u8,

    mcu_pbclk_div: u8,
    lp_div: u8,
    dsp_clk: u8,
    dsp_clk_div: u8,

    dsp_bclk_div: u8,
    dsp_pbclk: u8,
    dsp_pbclk_div: u8,
    emi_clk: u8,

    emi_clk_div: u8,
    flash_clk_type: u8,
    flash_clk_div: u8,
    wifipll_pu: u8,

    aupll_pu: u8,
    cpupll_pu: u8,
    mipipll_pu: u8,
    uhspll_pu: u8,
}

impl HalSysClkConfig {
    #[inline]
    pub const fn crc32(&self) -> u32 {
        let mut buf = [0u8; 20];

        buf[0] = self.xtal_type;
        buf[1] = self.mcu_clk;
        buf[2] = self.mcu_clk_div;
        buf[3] = self.mcu_bclk_div;

        buf[4] = self.mcu_pbclk_div;
        buf[5] = self.lp_div;
        buf[6] = self.dsp_clk;
        buf[7] = self.dsp_clk_div;

        buf[8] = self.dsp_bclk_div;
        buf[9] = self.dsp_pbclk;
        buf[10] = self.dsp_pbclk_div;
        buf[11] = self.emi_clk;

        buf[12] = self.emi_clk_div;
        buf[13] = self.flash_clk_type;
        buf[14] = self.flash_clk_div;
        buf[15] = self.wifipll_pu;

        buf[16] = self.aupll_pu;
        buf[17] = self.cpupll_pu;
        buf[18] = self.mipipll_pu;
        buf[19] = self.uhspll_pu;

        crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&buf)
    }
}

/// Clock configuration in ROM header.
#[repr(C)]
pub struct HalPllConfig {
    magic: u32,
    cfg: HalSysClkConfig,
    crc32: u32,
}

impl HalPllConfig {
    /// Create this structure with magic number and CRC32 filled in compile time.
    #[inline]
    pub const fn new(cfg: HalSysClkConfig) -> Self {
        let crc32 = cfg.crc32();
        HalPllConfig {
            magic: 0x47464350,
            cfg,
            crc32,
        }
    }
}

/// Processor core configuration in ROM header.
#[repr(C)]
pub struct HalCpuCfg {
    /// Config this cpu.
    config_enable: u8,
    /// Halt this cpu.
    halt_cpu: u8,
    /// Cache setting.
    cache_flags: u8,
    _rsvd: u8,
    /// Cache range high.
    cache_range_h: u32,
    /// Cache range low.
    cache_range_l: u32,
    /// Image address on flash.
    image_address_offset: u32,
    /// Entry point of the m0 image.
    boot_entry: u32,
    /// Msp value.
    msp_val: u32,
}

impl HalCpuCfg {
    #[allow(dead_code)]
    #[inline]
    const fn disabled() -> HalCpuCfg {
        HalCpuCfg {
            config_enable: 0,
            halt_cpu: 0,
            cache_flags: 0,
            _rsvd: 0,
            cache_range_h: 0,
            cache_range_l: 0,
            image_address_offset: 0,
            boot_entry: 0x58000000,
            msp_val: 0,
        }
    }
}

/// Peripherals available on ROM start.
pub struct Peripherals<'a> {
    /// Global configuration peripheral.
    pub glb: GLBv2,
    /// General Purpose Input/Output pads.
    pub gpio: bouffalo_hal::gpio::Pads<'a>,
    /// UART signal multiplexers.
    pub uart_muxes: bouffalo_hal::uart::UartMuxes<'a>,
    /// Universal Asynchronous Receiver/Transmitter peripheral 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter peripheral 1.
    pub uart1: UART1,
    /// Serial Peripheral Interface peripheral 0.
    pub spi0: SPI0,
    /// Inter-Integrated Circuit bus peripheral 0.
    pub i2c0: I2C0,
    /// Pulse Width Modulation peripheral.
    pub pwm: PWM,
    /// Inter-Integrated Circuit bus peripheral 1.
    pub i2c1: I2C1,
    /// Universal Asynchronous Receiver/Transmitter peripheral 2.
    pub uart2: UART2,
    /// Hardware LZ4 Decompressor.
    pub lz4d: LZ4D,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Ethernet Media Access Control peripheral.
    pub emac: EMAC,
    /// Universal Asynchronous Receiver/Transmitter peripheral 3.
    pub uart3: UART3,
    /// Inter-Integrated Circuit bus peripheral 2.
    pub i2c2: I2C2,
    /// Inter-Integrated Circuit bus peripheral 3.
    pub i2c3: I2C3,
    /// Serial Peripheral Interface peripheral 1.
    pub spi1: SPI1,
    /// Platform-local Interrupt Controller.
    pub plic: PLIC,
    /// Multi-media subsystem global peripheral.
    pub mmglb: MMGLB,
    /// Pseudo Static Random Access Memory controller.
    pub psram: PSRAM,
    /// Secure Digital High Capacity peripheral.
    pub sdh: SDH,
    /// Direct Memory Access peripheral 0.
    pub dma0: DMA0,
    /// Direct Memory Access peripheral 1.
    pub dma1: DMA1,
    /// Direct Memory Access peripheral 2.
    pub dma2: DMA2,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv2 => 0x20000000, bouffalo_hal::glb::v2::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0 with fixed base address.
    pub struct UART0 => 0x2000A000, bouffalo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1 with fixed base address.
    pub struct UART1 => 0x2000A100, bouffalo_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral 0.
    pub struct SPI0 => 0x2000A200, bouffalo_hal::spi::RegisterBlock;
    /// Inter-Integrated Circuit bus 0 with fixed base address.
    pub struct I2C0 => 0x2000A300, bouffalo_hal::i2c::RegisterBlock;
    /// Pulse Width Modulation peripheral.
    pub struct PWM => 0x2000A400, bouffalo_hal::pwm::RegisterBlock;
    /// Inter-Integrated Circuit bus 1 with fixed base address.
    pub struct I2C1 => 0x2000A900, bouffalo_hal::i2c::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 2 with fixed base address.
    pub struct UART2 => 0x2000AA00, bouffalo_hal::uart::RegisterBlock;
    /// Hardware LZ4 Decompressor.
    pub struct LZ4D => 0x2000AD00, bouffalo_hal::lz4d::RegisterBlock;
    /// Direct Memory Access peripheral 0.
    pub struct DMA0 => 0x2000C000, bouffalo_hal::dma::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x2000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Secure Digital High Capacity peripheral.
    pub struct SDH => 0x20060000, bouffalo_hal::sdio::RegisterBlock;
    /// Ethernet Media Access Control peripheral.
    pub struct EMAC => 0x20070000, bouffalo_hal::emac::RegisterBlock;
    /// Direct Memory Access peripheral 1.
    pub struct DMA1 => 0x20071000, bouffalo_hal::dma::RegisterBlock;
    /// Direct Memory Access peripheral 2.
    pub struct DMA2 => 0x30001000, bouffalo_hal::dma::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 3 with fixed base address.
    pub struct UART3 => 0x30002000, bouffalo_hal::uart::RegisterBlock;
    /// Inter-Integrated Circuit bus 2 with fixed base address.
    pub struct I2C2 => 0x30003000, bouffalo_hal::i2c::RegisterBlock;
    /// Inter-Integrated Circuit bus 3 with fixed base address.
    pub struct I2C3 => 0x30004000, bouffalo_hal::i2c::RegisterBlock;
    /// Multi-media subsystem global peripheral.
    pub struct MMGLB => 0x30007000, bouffalo_hal::glb::mm::RegisterBlock;
    /// Serial Peripheral Interface peripheral 1.
    pub struct SPI1 => 0x30008000, bouffalo_hal::spi::RegisterBlock;
    /// Pseudo Static Random Access Memory controller.
    pub struct PSRAM => 0x3000F000, bouffalo_hal::psram::RegisterBlock;
    /// Platform-local Interrupt Controller.
    pub struct PLIC => 0xE0000000, xuantie_riscv::peripheral::plic::Plic;
}

pub use bouffalo_hal::clocks::Clocks;
use bouffalo_hal::dma::{DmaExt, EightChannels, FourChannels, Periph4Dma01, Periph4Dma2};

impl<'a> DmaExt for &'a mut DMA0 {
    type Group = EightChannels<'a, Periph4Dma01>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        EightChannels::__new::<0>(self, glb)
    }
}

impl DmaExt for DMA0 {
    type Group = EightChannels<'static, Periph4Dma01>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        EightChannels::__new::<0>(unsafe { &*self.as_ptr() }, glb)
    }
}

impl<'a> DmaExt for &'a mut DMA1 {
    type Group = FourChannels<'a, Periph4Dma01>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        FourChannels::__new::<1>(self, glb)
    }
}

impl DmaExt for DMA1 {
    type Group = FourChannels<'static, Periph4Dma01>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        FourChannels::__new::<1>(unsafe { &*self.as_ptr() }, glb)
    }
}

impl<'a> DmaExt for &'a mut DMA2 {
    type Group = EightChannels<'a, Periph4Dma2>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        EightChannels::__new::<2>(self, glb)
    }
}

impl DmaExt for DMA2 {
    type Group = EightChannels<'static, Periph4Dma2>;

    #[inline]
    fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
        EightChannels::__new::<2>(unsafe { &*self.as_ptr() }, glb)
    }
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params(xtal_hz: u32) -> (Peripherals<'static>, Clocks) {
    use embedded_time::rate::Hertz;
    let peripherals = Peripherals {
        glb: GLBv2 { _private: () },
        gpio: match () {
            #[cfg(any(feature = "bl808-dsp", feature = "bl808-mcu", feature = "bl808-lp"))]
            () => bouffalo_hal::gpio::Pads::__pads_from_glb(&GLBv2 { _private: () }),
            #[cfg(not(any(feature = "bl808-dsp", feature = "bl808-mcu", feature = "bl808-lp")))]
            () => unimplemented!(),
        },
        uart_muxes: bouffalo_hal::uart::UartMuxes::__uart_muxes_from_glb(&GLBv2 { _private: () }),
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        spi0: SPI0 { _private: () },
        i2c0: I2C0 { _private: () },
        pwm: PWM { _private: () },
        i2c1: I2C1 { _private: () },
        uart2: UART2 { _private: () },
        lz4d: LZ4D { _private: () },
        hbn: HBN { _private: () },
        emac: EMAC { _private: () },
        uart3: UART3 { _private: () },
        i2c2: I2C2 { _private: () },
        i2c3: I2C3 { _private: () },
        spi1: SPI1 { _private: () },
        plic: PLIC { _private: () },
        mmglb: MMGLB { _private: () },
        psram: PSRAM { _private: () },
        sdh: SDH { _private: () },
        dma0: DMA0 { _private: () },
        dma1: DMA1 { _private: () },
        dma2: DMA2 { _private: () },
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}

#[cfg(test)]
mod tests {
    use super::{HalBootheader, HalCpuCfg, HalPllConfig, HalSysClkConfig};
    use core::mem::offset_of;

    #[test]
    fn struct_lengths() {
        use core::mem::size_of;
        assert_eq!(size_of::<HalPllConfig>(), 28);
        assert_eq!(size_of::<HalBootheader>(), 352);
    }

    #[test]
    fn struct_hal_bootheader_offset() {
        assert_eq!(offset_of!(HalBootheader, magic), 0x00);
        assert_eq!(offset_of!(HalBootheader, revision), 0x04);
        assert_eq!(offset_of!(HalBootheader, flash_cfg), 0x08);
        assert_eq!(offset_of!(HalBootheader, clk_cfg), 0x64);
        assert_eq!(offset_of!(HalBootheader, basic_cfg), 0x80);
        assert_eq!(offset_of!(HalBootheader, cpu_cfg), 0xb0);
        assert_eq!(offset_of!(HalBootheader, boot2_pt_table_0), 0xf8);
        assert_eq!(offset_of!(HalBootheader, boot2_pt_table_1), 0xfc);
        assert_eq!(offset_of!(HalBootheader, flash_cfg_table_addr), 0x100);
        assert_eq!(offset_of!(HalBootheader, flash_cfg_table_len), 0x104);
        assert_eq!(offset_of!(HalBootheader, patch_on_read), 0x108);
        assert_eq!(offset_of!(HalBootheader, patch_on_jump), 0x128);
        assert_eq!(offset_of!(HalBootheader, crc32), 0x15c);
    }

    #[test]
    fn struct_hal_sys_clk_config_offset() {
        assert_eq!(offset_of!(HalSysClkConfig, xtal_type), 0x00);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_clk), 0x01);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_clk_div), 0x02);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_bclk_div), 0x03);

        assert_eq!(offset_of!(HalSysClkConfig, mcu_pbclk_div), 0x04);
        assert_eq!(offset_of!(HalSysClkConfig, lp_div), 0x05);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_clk), 0x06);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_clk_div), 0x07);

        assert_eq!(offset_of!(HalSysClkConfig, dsp_bclk_div), 0x08);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_pbclk), 0x9);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_pbclk_div), 0x0a);
        assert_eq!(offset_of!(HalSysClkConfig, emi_clk), 0x0b);

        assert_eq!(offset_of!(HalSysClkConfig, emi_clk_div), 0x0c);
        assert_eq!(offset_of!(HalSysClkConfig, flash_clk_type), 0x0d);
        assert_eq!(offset_of!(HalSysClkConfig, flash_clk_div), 0x0e);
        assert_eq!(offset_of!(HalSysClkConfig, wifipll_pu), 0x0f);

        assert_eq!(offset_of!(HalSysClkConfig, aupll_pu), 0x10);
        assert_eq!(offset_of!(HalSysClkConfig, cpupll_pu), 0x11);
        assert_eq!(offset_of!(HalSysClkConfig, mipipll_pu), 0x12);
        assert_eq!(offset_of!(HalSysClkConfig, uhspll_pu), 0x13);
    }

    #[test]
    fn struct_hal_pll_config_offset() {
        assert_eq!(offset_of!(HalPllConfig, magic), 0x00);
        assert_eq!(offset_of!(HalPllConfig, cfg), 0x04);
        assert_eq!(offset_of!(HalPllConfig, crc32), 0x18);
    }

    #[test]
    fn struct_hal_cpu_cfg_offset() {
        assert_eq!(offset_of!(HalCpuCfg, config_enable), 0x00);
        assert_eq!(offset_of!(HalCpuCfg, halt_cpu), 0x01);
        assert_eq!(offset_of!(HalCpuCfg, cache_flags), 0x02);
        assert_eq!(offset_of!(HalCpuCfg, cache_range_h), 0x04);
        assert_eq!(offset_of!(HalCpuCfg, cache_range_l), 0x08);
        assert_eq!(offset_of!(HalCpuCfg, image_address_offset), 0x0c);
        assert_eq!(offset_of!(HalCpuCfg, boot_entry), 0x10);
        assert_eq!(offset_of!(HalCpuCfg, msp_val), 0x14);
    }

    #[test]
    fn magic_crc32_hal_pll_config() {
        let test_sys_clk_config = HalSysClkConfig {
            xtal_type: 7,
            mcu_clk: 4,
            mcu_clk_div: 0,
            mcu_bclk_div: 0,
            mcu_pbclk_div: 3,
            lp_div: 1,
            dsp_clk: 3,
            dsp_clk_div: 0,
            dsp_bclk_div: 1,
            dsp_pbclk: 2,
            dsp_pbclk_div: 0,
            emi_clk: 2,
            emi_clk_div: 1,
            flash_clk_type: 1,
            flash_clk_div: 0,
            wifipll_pu: 1,
            aupll_pu: 1,
            cpupll_pu: 1,
            mipipll_pu: 1,
            uhspll_pu: 1,
        };
        let test_config = HalPllConfig::new(test_sys_clk_config);
        assert_eq!(test_config.magic, 0x47464350);
        assert_eq!(test_config.crc32, 0x864b890a);
    }
}
