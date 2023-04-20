use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("linker.ld"); // 脚本文件名可以填写任何名字，因为rustc会按目录搜索

    std::fs::write(ld, LINKER_SCRIPT).unwrap();
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start) 
MEMORY {
    FLASH : ORIGIN = 0x58000000, LENGTH = 32M
    DRAM : ORIGIN = 0x3EF80000, LENGTH = 512K 
}
SECTIONS {
    .text : ALIGN(8) {  
        *(.text .text.*)
    } > FLASH
    .rodata : ALIGN(8) { 
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);  
        erodata = .;
    } > DRAM  
    .data : ALIGN(8) { 
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8); 
        edata = .;
    } > DRAM 
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(8) {  
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > DRAM  
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
