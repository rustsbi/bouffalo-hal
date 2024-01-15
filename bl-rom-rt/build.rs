use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("bl-rom-rt.ld");

    #[cfg(feature = "bl616")]
    std::fs::write(ld, LINKER_SCRIPT_BL616).unwrap();
    #[cfg(feature = "bl808-mcu")]
    std::fs::write(ld, LINKER_SCRIPT_BL808_MCU).unwrap();
    #[cfg(feature = "bl808-dsp")]
    std::fs::write(ld, LINKER_SCRIPT_BL808_DSP).unwrap();
    #[cfg(feature = "bl702")]
    std::fs::write(ld, LINKER_SCRIPT_BL702).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

#[cfg(feature = "bl616")]
const LINKER_SCRIPT_BL616: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0xA0000000 - 0x1000, LENGTH = 4K
    FLASH : ORIGIN = 0xA0000000, LENGTH = 4M - 4K
    OCRAM : ORIGIN = 0x62FC0000, LENGTH = 320K
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
        LONG(SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data));
        KEEP(*(.head.base.hash));
        KEEP(*(.head.cpu));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
        KEEP(*(.head.patch.on-read));
        KEEP(*(.head.patch.on-jump));
        LONG(0);
        KEEP(*(.head.crc32));
        FILL(0xFFFFFFFF);
        . = ORIGIN(PSEUDO_HEADER) + LENGTH(PSEUDO_HEADER);
    } > PSEUDO_HEADER
    .text : ALIGN(4) {
        stext = .;
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(4);
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
    } > OCRAM AT>FLASH
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > OCRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

#[cfg(feature = "bl808-mcu")]
const LINKER_SCRIPT_BL808_MCU: &[u8] = b"
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
        LONG(SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data));
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
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(4);
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

#[cfg(feature = "bl808-dsp")]
const LINKER_SCRIPT_BL808_DSP: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start) 
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0x58000000 - 0x1000, LENGTH = 4K
    FLASH : ORIGIN = 0x58000000, LENGTH = 32M - 4K
    DRAM : ORIGIN = 0x3EFF7000, LENGTH = 4K
    VRAM : ORIGIN = 0x3F000000, LENGTH = 32K
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
        LONG(SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data));
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
        stext = .;
        KEEP(*(.text.entry))
        . = ALIGN(4);
        *(.trap.trap-entry)
        *(.text .text.*)
        . = ALIGN(8);
        etext = .;
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
    } > VRAM AT>FLASH
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(8) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > VRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}
/* exceptions */
PROVIDE(exceptions = default_handler);
/* interrupts */
PROVIDE(bmx_dsp_bus_err = default_handler);
PROVIDE(dsp_reserved1 = default_handler);
PROVIDE(dsp_reserved2 = default_handler);
PROVIDE(dsp_reserved3 = default_handler);
PROVIDE(uart3 = default_handler);
PROVIDE(i2c2 = default_handler);
PROVIDE(i2c3 = default_handler);
PROVIDE(spi1 = default_handler);
PROVIDE(dsp_reserved4 = default_handler);
PROVIDE(dsp_reserved5 = default_handler);
PROVIDE(seof_int0 = default_handler);
PROVIDE(seof_int1 = default_handler);
PROVIDE(seof_int2 = default_handler);
PROVIDE(dvp2_bus_int0 = default_handler);
PROVIDE(dvp2_bus_int1 = default_handler);
PROVIDE(dvp2_bus_int2 = default_handler);
PROVIDE(dvp2_bus_int3 = default_handler);
PROVIDE(h264_bs = default_handler);
PROVIDE(h264_frame = default_handler);
PROVIDE(h264_seq_done = default_handler);
PROVIDE(mjpeg = default_handler);
PROVIDE(h264_s_bs = default_handler);
PROVIDE(h264_s_frame = default_handler);
PROVIDE(h264_s_seq_done = default_handler);
PROVIDE(dma2_int0 = default_handler);
PROVIDE(dma2_int1 = default_handler);
PROVIDE(dma2_int2 = default_handler);
PROVIDE(dma2_int3 = default_handler);
PROVIDE(dma2_int4 = default_handler);
PROVIDE(dma2_int5 = default_handler);
PROVIDE(dma2_int6 = default_handler);
PROVIDE(dma2_int7 = default_handler);
PROVIDE(dsp_reserved6 = default_handler);
PROVIDE(dsp_reserved7 = default_handler);
PROVIDE(dsp_reserved8 = default_handler);
PROVIDE(dsp_reserved9 = default_handler);
PROVIDE(dsp_reserved10 = default_handler);
PROVIDE(mipi_csi = default_handler);
PROVIDE(ipc_d0 = default_handler);
PROVIDE(dsp_reserved11 = default_handler);
PROVIDE(mjdec = default_handler);
PROVIDE(dvp2_bus_int4 = default_handler);
PROVIDE(dvp2_bus_int5 = default_handler);
PROVIDE(dvp2_bus_int6 = default_handler);
PROVIDE(dvp2_bus_int7 = default_handler);
PROVIDE(dma2_d_int0 = default_handler);
PROVIDE(dma2_d_int1 = default_handler);
PROVIDE(display = default_handler);
PROVIDE(pwm = default_handler);
PROVIDE(seof_int3 = default_handler);
PROVIDE(dsp_reserved12 = default_handler);
PROVIDE(dsp_reserved13 = default_handler);
PROVIDE(osd = default_handler);
PROVIDE(dbi = default_handler);
PROVIDE(dsp_reserved14 = default_handler);
PROVIDE(osda_bus_drain = default_handler);
PROVIDE(osdb_bus_drain = default_handler);
PROVIDE(osd_pb = default_handler);
PROVIDE(dsp_reserved15 = default_handler);
PROVIDE(mipi_dsi = default_handler);
PROVIDE(dsp_reserved16 = default_handler);
PROVIDE(timer0 = default_handler);
PROVIDE(timer1 = default_handler);
PROVIDE(wdt = default_handler);
PROVIDE(audio = default_handler);
PROVIDE(wl_all = default_handler);
PROVIDE(pds = default_handler);
";

#[cfg(feature = "bl702")]
const LINKER_SCRIPT_BL702: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0x23000000 - 0x1000, LENGTH = 4K
    XIP : ORIGIN = 0x23000000, LENGTH = 8M - 4K
    OCRAM : ORIGIN = 0x22020000, LENGTH = 64K
}
SECTIONS {
    .head : ALIGN(4) {
        LONG(0x504E4642);
        LONG(1);
        KEEP(*(.head.flash));
        KEEP(*(.head.clock));
        KEEP(*(.head.base.flag));
        LONG(SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data));
        LONG(0);
        LONG(ADDR(.text) - ORIGIN(PSEUDO_HEADER));
        KEEP(*(.head.base.hash));
        LONG(0x00001000);
        LONG(0x00002000);
        KEEP(*(.head.crc32));
        FILL(0xFFFFFFFF);
        . = ORIGIN(PSEUDO_HEADER) + LENGTH(PSEUDO_HEADER);
    } > PSEUDO_HEADER
    .text : ALIGN(4) {
        stext = .;
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(4);
        etext = .;
    } > XIP
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > XIP
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > OCRAM AT>XIP
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > OCRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
