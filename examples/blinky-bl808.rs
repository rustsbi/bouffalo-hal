// Build this example with:
// m0:
// rustup target install riscv32imac-unknown-none-elf
// cargo build --example blinky-bl808 --features bl808-m0 --target riscv32imac-unknown-none-elf --release
// d0:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --example blinky-bl808 --features bl808-d0 --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::entry;
use core::{arch::asm, ptr};

#[entry]
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

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
