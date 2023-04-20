#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]

use core::arch::asm;
use core::ptr;
use core::panic::PanicInfo;

const LEN_STACK_D0: usize = 1 * 1024;

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

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; LEN_STACK_D0] = [0; LEN_STACK_D0];
    asm!(
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
        "	la  	t1, sbss
        	la   	t2, ebss
    	1:  bgeu 	t1, t2, 1f
        	sd   	zero, 0(t1) 
        	addi 	t1, t1, 8 
        	j    	1b
    	1:",
        "   call    {main}", 
        stack = sym STACK,
        hart_stack_size = const LEN_STACK_D0,
        main = sym main,
        options(noreturn)
    )
}
