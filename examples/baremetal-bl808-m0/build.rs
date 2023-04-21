use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("linker.ld");

    std::fs::write(ld, LINKER_SCRIPT).unwrap();
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start) 
MEMORY {
FLASH : ORIGIN = 0x58000000, LENGTH = 32M
    WRAM : ORIGIN = 0x62030000, LENGTH = 160K 
}
SECTIONS {
    .text : ALIGN(4) { 
        *(.text .text.*)
    } > FLASH
    .rodata : ALIGN(4) { 
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4); 
        erodata = .;
    } > WRAM 
    .data : ALIGN(4) { 
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4); 
        edata = .;
    } > WRAM 
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) { 
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > WRAM 
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
