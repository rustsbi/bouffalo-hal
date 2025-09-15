//! BL808 tri-core heterogeneous Wi-Fi 802.11b/g/n, Bluetooth 5, Zigbee AIoT system-on-chip.

pub mod image_header;
mod peripheral;
pub use image_header::*;
pub use peripheral::*;

#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
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

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
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

#[cfg(all(feature = "bl808-lp", target_arch = "riscv32"))]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
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
#[unsafe(naked)]
unsafe extern "C" fn trap_vectored() -> ! {
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

#[cfg(any(
    all(feature = "bl808-mcu", target_arch = "riscv32"),
    all(feature = "bl808-dsp", target_arch = "riscv64")
))]
#[unsafe(naked)]
unsafe extern "C" fn reserved() -> ! {
    core::arch::naked_asm!("1: j   1b")
}

#[cfg(any(all(feature = "bl808-dsp", target_arch = "riscv64")))]
unsafe extern "C" {
    fn exceptions(tf: &mut crate::arch::rvi::TrapFrame);
}

// TODO exceptions_trampoline for bl808-mcu
#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[unsafe(naked)]
unsafe extern "C" fn exceptions_trampoline() -> ! {
    unsafe { core::arch::naked_asm!("") }
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[unsafe(naked)]
unsafe extern "C" fn exceptions_trampoline() -> ! {
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

// TODO machine_external_trampoline for bl808-mcu
#[cfg(all(feature = "bl808-mcu", target_arch = "riscv32"))]
#[unsafe(naked)]
unsafe extern "C" fn machine_external_trampoline() -> ! {
    core::arch::naked_asm!("")
}

#[cfg(all(feature = "bl808-dsp", target_arch = "riscv64"))]
#[unsafe(naked)]
unsafe extern "C" fn machine_external_trampoline() -> ! {
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
