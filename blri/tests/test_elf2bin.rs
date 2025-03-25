use blri::elf_to_bin_bytes;

macro_rules! test_elf2bin {
    ($name:ident, $elf:expr, $rust_objcopy_bin:expr) => {
        #[test]
        fn $name() {
            let elf = include_bytes!($elf);
            let rust_objcopy_bin = include_bytes!($rust_objcopy_bin);
            let bin = elf_to_bin_bytes(elf).expect("convert elf to bin");
            assert_eq!(bin, rust_objcopy_bin);
        }
    };
    () => {};
}

// GPIO Demo
test_elf2bin!(
    test_gpio_demo,
    "elf2bin/elf/gpio-demo",
    "elf2bin/rust-objcopy-bin/gpio-demo.bin"
);

// I2C Demo
test_elf2bin!(
    test_i2c_demo,
    "elf2bin/elf/i2c-demo",
    "elf2bin/rust-objcopy-bin/i2c-demo.bin"
);

// JTAG Demo
test_elf2bin!(
    test_jtag_demo,
    "elf2bin/elf/jtag-demo",
    "elf2bin/rust-objcopy-bin/jtag-demo.bin"
);

// LZ4D Demo
test_elf2bin!(
    test_lz4d_demo,
    "elf2bin/elf/lz4d-demo",
    "elf2bin/rust-objcopy-bin/lz4d-demo.bin"
);

// PSRAM Demo
test_elf2bin!(
    test_psram_demo,
    "elf2bin/elf/psram-demo",
    "elf2bin/rust-objcopy-bin/psram-demo.bin"
);

// PWM Demo
test_elf2bin!(
    test_pwm_demo,
    "elf2bin/elf/pwm-demo",
    "elf2bin/rust-objcopy-bin/pwm-demo.bin"
);

// SDCard Demo
test_elf2bin!(
    test_sdcard_demo,
    "elf2bin/elf/sdcard-demo",
    "elf2bin/rust-objcopy-bin/sdcard-demo.bin"
);

// SDCard GPT Demo
test_elf2bin!(
    test_sdcard_gpt_demo,
    "elf2bin/elf/sdcard-gpt-demo",
    "elf2bin/rust-objcopy-bin/sdcard-gpt-demo.bin"
);

// SDH Demo
test_elf2bin!(
    test_sdh_demo,
    "elf2bin/elf/sdh-demo",
    "elf2bin/rust-objcopy-bin/sdh-demo.bin"
);

// SPI Demo
test_elf2bin!(
    test_spi_demo,
    "elf2bin/elf/spi-demo",
    "elf2bin/rust-objcopy-bin/spi-demo.bin"
);

// UART Async Demo
test_elf2bin!(
    test_uart_async_demo,
    "elf2bin/elf/uart-async-demo",
    "elf2bin/rust-objcopy-bin/uart-async-demo.bin"
);

// UART CLI Demo
test_elf2bin!(
    test_uart_cli_demo,
    "elf2bin/elf/uart-cli-demo",
    "elf2bin/rust-objcopy-bin/uart-cli-demo.bin"
);

// UART Demo
test_elf2bin!(
    test_uart_demo,
    "elf2bin/elf/uart-demo",
    "elf2bin/rust-objcopy-bin/uart-demo.bin"
);

// UART DMA Demo
test_elf2bin!(
    test_uart_dma_demo,
    "elf2bin/elf/uart-dma-demo",
    "elf2bin/rust-objcopy-bin/uart-dma-demo.bin"
);
