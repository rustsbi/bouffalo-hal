//  cargo build --example blinky-bl808-m0 --features bl808-m0 --target riscv32imac-unknown-none-elf --release

#![feature(naked_functions, asm_const)]
#![no_std]
#![no_main]
use bl_rom_rt::BOOTHEADER;
use core::arch::asm;
use core::ptr;

const LEN_STACK_M0: usize = 1 * 1024;

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; LEN_STACK_M0] = [0; LEN_STACK_M0];
    asm!(
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
        "   la      t1, sbss
            la      t2, ebss
    	1:  bgeu    t1, t2, 1f
            sw      zero, 0(t1)
            addi    t1, t1, 4
            j       1b
            la      a0, {}
        1:",
        "   call    {main}",
        sym BOOTHEADER,
        stack = sym STACK,
        hart_stack_size = const LEN_STACK_M0,
        main = sym main,
        options(noreturn)
    )
}

fn main() -> ! {
    unsafe {
        ptr::write_volatile(
            0x200008e4 as *mut u32,
            (ptr::read_volatile(0x200008e4 as *mut u32) & 0x3fffffae) | 0x40000050,
        )
    };
    unsafe {
        ptr::write_volatile(
            0x200008e4 as *mut u32,
            ptr::read_volatile(0x200008e4 as *mut u32) & 0xfdffffff,
        )
    };
    loop {
        for _ in 0..100_000 {
            unsafe { asm!("nop") };
        }
        unsafe {
            ptr::write_volatile(
                0x200008e4 as *mut u32,
                ptr::read_volatile(0x200008e4 as *mut u32) | 0x02000000,
            )
        };
        for _ in 0..100_000 {
            unsafe { asm!("nop") };
        }
        unsafe {
            ptr::write_volatile(
                0x200008e4 as *mut u32,
                ptr::read_volatile(0x200008e4 as *mut u32) | 0x04000000,
            )
        };
    }
}

#[cfg_attr(test, allow(unused))]
#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
