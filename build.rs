use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("linker.ld");

    #[cfg(feature = "bl808-m0")]
    std::fs::write(ld, LINKER_SCRIPT).unwrap();
    #[cfg(feature = "bl808-d0")]
    std::fs::write(ld, LINKER_SCRIPT).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

#[cfg(feature = "bl808-m0")]
const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0x58000000 - 0x1000, LENGTH = 4K
    FLASH : ORIGIN = 0x58000000, LENGTH = 32M - 4K
    WRAM : ORIGIN = 0x62030000, LENGTH = 160K
}
SECTIONS {
    .head : ALIGN(4) { 
        LONG(0x504E4642);
        LONG(1);
        KEEP(*(.head.flash));
        KEEP(*(.head.clock));
        KEEP(*(.head.base.flag));
        LONG(ADDR(.text) - ORIGIN(PSEUDO_HEADER));
        KEEP(*(.head.base.aes-region));
        LONG(SIZEOF(.text));
        KEEP(*(.head.base.hash));
        KEEP(*(.head.cpu));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        KEEP(*(.head.patch.on-read));
        KEEP(*(.head.patch.on-jump));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        KEEP(*(.head.crc32));
        FILL(0xFFFFFFFF);
        . = ORIGIN(PSEUDO_HEADER) + LENGTH(PSEUDO_HEADER);
    } > PSEUDO_HEADER
    .text : ALIGN(4) {
        stext = .;
        *(.text .text.*)
        etext = .;
    } > FLASH
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > FLASH
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > WRAM AT>FLASH
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

#[cfg(feature = "bl808-d0")]
const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start) 
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0x58000000 - 0x1000, LENGTH = 4K
    FLASH : ORIGIN = 0x58000000, LENGTH = 32M
    DRAM : ORIGIN = 0x3EF80000, LENGTH = 512K 
}
SECTIONS {
    .head : ALIGN(8) { 
        LONG(0x504E4642);
        LONG(1);
        KEEP(*(.head.flash));
        KEEP(*(.head.clock));
        KEEP(*(.head.base.flag));
        LONG(ADDR(.text) - ORIGIN(PSEUDO_HEADER));
        KEEP(*(.head.base.aes-region));
        LONG(SIZEOF(.text));
        KEEP(*(.head.base.hash));
        KEEP(*(.head.cpu));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        KEEP(*(.head.patch.on-read));
        KEEP(*(.head.patch.on-jump));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        KEEP(*(.head.crc32));
        FILL(0xFFFFFFFF);
        . = ORIGIN(PSEUDO_HEADER) + LENGTH(PSEUDO_HEADER);
    } > PSEUDO_HEADER
    .text : ALIGN(8) {  
        *(.text .text.*)
    } > FLASH
    .rodata : ALIGN(8) { 
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);  
        erodata = .;
    } > FLASH  
    .data : ALIGN(8) { 
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8); 
        edata = .;
    } > DRAM AT>FLASH
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
