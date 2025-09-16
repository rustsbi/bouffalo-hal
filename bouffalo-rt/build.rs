fn main() {
    let (out, ld) = {
        use std::{env, path::PathBuf};
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let ld = out.join("bouffalo-rt.ld");
        (out, ld)
    };

    #[cfg(feature = "bl616")]
    std::fs::write(&ld, LINKER_SCRIPT_BL616).unwrap();
    #[cfg(feature = "bl808-mcu")]
    std::fs::write(&ld, LINKER_SCRIPT_BL808_MCU).unwrap();
    #[cfg(feature = "bl808-dsp")]
    std::fs::write(&ld, LINKER_SCRIPT_BL808_DSP).unwrap();
    #[cfg(feature = "bl808-lp")]
    std::fs::write(&ld, LINKER_SCRIPT_BL808_LP).unwrap();
    #[cfg(feature = "bl702")]
    std::fs::write(&ld, LINKER_SCRIPT_BL702).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    let _ = (ld, out);
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
}
/* exceptions */
PROVIDE(exceptions = default_handler);
/* interrupts */
PROVIDE(bmx_mcu_bus_err = default_handler);
PROVIDE(bmx_mcu_to = default_handler);
PROVIDE(m0_reserved2 = default_handler);
PROVIDE(ipc_m0 = default_handler);
PROVIDE(audio = default_handler);
PROVIDE(rf_top_int0 = default_handler);
PROVIDE(rf_top_int1 = default_handler);
PROVIDE(lz4d = default_handler);
PROVIDE(gauge_itf = default_handler);
PROVIDE(sec_eng_id1_sha_aes_trng_pka_gmac = default_handler);
PROVIDE(sec_eng_id0_sha_aes_trng_pka_gmac = default_handler);
PROVIDE(sec_eng_id1_cdet = default_handler);
PROVIDE(sec_eng_id0_cdet = default_handler);
PROVIDE(sf_ctrl_id1 = default_handler);
PROVIDE(sf_ctrl_id0 = default_handler);
PROVIDE(dma0_all = default_handler);
PROVIDE(dma1_all = default_handler);
PROVIDE(sdh = default_handler);
PROVIDE(mm_all = default_handler);
PROVIDE(irtx = default_handler);
PROVIDE(irrx = default_handler);
PROVIDE(usb = default_handler);
PROVIDE(aupdm_touch = default_handler);
PROVIDE(m0_reserved23 = default_handler);
PROVIDE(emac = default_handler);
PROVIDE(gpadc_dma = default_handler);
PROVIDE(efuse = default_handler);
PROVIDE(spi0 = default_handler);
PROVIDE(uart0 = default_handler);
PROVIDE(uart1 = default_handler);
PROVIDE(uart2 = default_handler);
PROVIDE(gpio_dma = default_handler);
PROVIDE(i2c0 = default_handler);
PROVIDE(pwm = default_handler);
PROVIDE(ipc_rsvd = default_handler);
PROVIDE(ipc_lp = default_handler);
PROVIDE(timer0_ch0 = default_handler);
PROVIDE(timer0_ch1 = default_handler);
PROVIDE(timer0_wdt = default_handler);
PROVIDE(i2c1 = default_handler);
PROVIDE(i2s = default_handler);
PROVIDE(ana_ocp_out_to_cpu_0 = default_handler);
PROVIDE(ana_ocp_out_to_cpu_1 = default_handler);
PROVIDE(ana_ocp_out_to_cpu_2 = default_handler);
PROVIDE(gpio_int0 = default_handler);
PROVIDE(dm = default_handler);
PROVIDE(bt = default_handler);
PROVIDE(m154_req_ack = default_handler);
PROVIDE(m154_int = default_handler);
PROVIDE(m154_aes = default_handler);
PROVIDE(pds_wakeup = default_handler);
PROVIDE(hbn_out0 = default_handler);
PROVIDE(hbn_out1 = default_handler);
PROVIDE(bor = default_handler);
PROVIDE(wifi = default_handler);
PROVIDE(bz_phy_int = default_handler);
PROVIDE(ble = default_handler);
PROVIDE(mac_txrx_timer = default_handler);
PROVIDE(mac_txrx_misc = default_handler);
PROVIDE(mac_rx_trg = default_handler);
PROVIDE(mac_tx_trg = default_handler);
PROVIDE(mac_gen = default_handler);
PROVIDE(mac_port_trg = default_handler);
PROVIDE(wifi_ipc_public = default_handler);
";

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

#[cfg(feature = "bl808-lp")]
const LINKER_SCRIPT_BL808_LP: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    PSEUDO_HEADER : ORIGIN = 0x58020000 - 0x1000, LENGTH = 4K
    FLASH : ORIGIN = 0x58020000, LENGTH = 1M - 4K
    RAM : ORIGIN = 0x22034000, LENGTH = 16K
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
    } > RAM AT>FLASH
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > RAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}
/* exceptions */
PROVIDE(exceptions = default_handler);
/* interrupts */
PROVIDE(bmx_mcu_bus_err = default_handler);
PROVIDE(bmx_mcu_to = default_handler);
PROVIDE(m0_reserved2 = default_handler);
PROVIDE(ipc_m0 = default_handler);
PROVIDE(audio = default_handler);
PROVIDE(rf_top_int0 = default_handler);
PROVIDE(rf_top_int1 = default_handler);
PROVIDE(lz4d = default_handler);
PROVIDE(gauge_itf = default_handler);
PROVIDE(sec_eng_id1_sha_aes_trng_pka_gmac = default_handler);
PROVIDE(sec_eng_id0_sha_aes_trng_pka_gmac = default_handler);
PROVIDE(sec_eng_id1_cdet = default_handler);
PROVIDE(sec_eng_id0_cdet = default_handler);
PROVIDE(sf_ctrl_id1 = default_handler);
PROVIDE(sf_ctrl_id0 = default_handler);
PROVIDE(dma0_all = default_handler);
PROVIDE(dma1_all = default_handler);
PROVIDE(sdh = default_handler);
PROVIDE(mm_all = default_handler);
PROVIDE(irtx = default_handler);
PROVIDE(irrx = default_handler);
PROVIDE(usb = default_handler);
PROVIDE(aupdm_touch = default_handler);
PROVIDE(m0_reserved23 = default_handler);
PROVIDE(emac = default_handler);
PROVIDE(gpadc_dma = default_handler);
PROVIDE(efuse = default_handler);
PROVIDE(spi0 = default_handler);
PROVIDE(uart0 = default_handler);
PROVIDE(uart1 = default_handler);
PROVIDE(uart2 = default_handler);
PROVIDE(gpio_dma = default_handler);
PROVIDE(i2c0 = default_handler);
PROVIDE(pwm = default_handler);
PROVIDE(ipc_rsvd = default_handler);
PROVIDE(ipc_lp = default_handler);
PROVIDE(timer0_ch0 = default_handler);
PROVIDE(timer0_ch1 = default_handler);
PROVIDE(timer0_wdt = default_handler);
PROVIDE(i2c1 = default_handler);
PROVIDE(i2s = default_handler);
PROVIDE(ana_ocp_out_to_cpu_0 = default_handler);
PROVIDE(ana_ocp_out_to_cpu_1 = default_handler);
PROVIDE(ana_ocp_out_to_cpu_2 = default_handler);
PROVIDE(gpio_int0 = default_handler);
PROVIDE(dm = default_handler);
PROVIDE(bt = default_handler);
PROVIDE(m154_req_ack = default_handler);
PROVIDE(m154_int = default_handler);
PROVIDE(m154_aes = default_handler);
PROVIDE(pds_wakeup = default_handler);
PROVIDE(hbn_out0 = default_handler);
PROVIDE(hbn_out1 = default_handler);
PROVIDE(bor = default_handler);
PROVIDE(wifi = default_handler);
PROVIDE(bz_phy_int = default_handler);
PROVIDE(ble = default_handler);
PROVIDE(mac_txrx_timer = default_handler);
PROVIDE(mac_txrx_misc = default_handler);
PROVIDE(mac_rx_trg = default_handler);
PROVIDE(mac_tx_trg = default_handler);
PROVIDE(mac_gen = default_handler);
PROVIDE(mac_port_trg = default_handler);
PROVIDE(wifi_ipc_public = default_handler);
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
