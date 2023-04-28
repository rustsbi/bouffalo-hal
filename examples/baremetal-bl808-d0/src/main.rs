#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]

use core::arch::asm;
use core::ptr;
use core::panic::PanicInfo;

#[bl_rom_rt::entry]
fn main() -> ! {
    unsafe { ptr::write_volatile(0x200008e4 as *mut u32,  
        (ptr::read_volatile(0x200008e4 as *mut u32) & 0x3fffffae) | 0x40000050) };
    unsafe { ptr::write_volatile(0x200008e4 as *mut u32, 
        ptr::read_volatile(0x200008e4 as *mut u32) & 0xfdffffff) };
    loop {
        for _ in 0..200_000 {
            unsafe { asm!("nop") };
        }
        unsafe { ptr::write_volatile(0x200008e4 as *mut u32, 
            ptr::read_volatile(0x200008e4 as *mut u32) | 0x02000000) };
        for _ in 0..200_000 {
            unsafe { asm!("nop") };
        }
        unsafe { ptr::write_volatile(0x200008e4 as *mut u32, 
            ptr::read_volatile(0x200008e4 as *mut u32) | 0x04000000) };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    loop {}
}
