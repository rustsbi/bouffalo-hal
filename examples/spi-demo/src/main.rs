// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p spi-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{gpio::Pins, prelude::*, spi::Spi, GLB, SPI};
use embedded_hal::spi::SpiDevice;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: Pins<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let spi: SPI<Static<0x30008000>> = unsafe { core::mem::transmute(()) };

    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let mut led = gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;

    let spi_cs = gpio.io12.into_spi::<1>();
    let spi_mosi = gpio.io25.into_spi::<1>();
    let spi_clk = gpio.io19.into_spi::<1>();
    let mut lcd_dc = gpio.io13.into_floating_output();
    let mut lcd_bl = gpio.io11.into_floating_output();
    let mut lcd_rst = gpio.io24.into_floating_output();
    let mut lcd = Spi::new(
        spi,
        (spi_clk, spi_mosi, spi_cs),
        embedded_hal::spi::MODE_0,
        &glb,
    );
    let mut data = [0u8; 4];

    lcd_bl.set_high().ok();
    lcd_rst.set_low().ok();
    unsafe { riscv::asm::delay(1000) }
    lcd_rst.set_high().ok();

    // lcd init
    lcd_dc.set_low().ok();
    lcd.write(&0x01_u8.to_be_bytes()).ok(); // SOFTWARE_RESET
    lcd.write(&0x11_u8.to_be_bytes()).ok(); // SLEEP_OFF
    lcd.write(&0x29_u8.to_be_bytes()).ok(); // DISPALY_ON
    lcd.write(&0x3A_u8.to_be_bytes()).ok(); // PIXEL_FORMAT_SET
    lcd_dc.set_high().ok();
    lcd.write(&0x55_u8.to_be_bytes()).ok();
    lcd_dc.set_low().ok();
    lcd.write(&0x36_u8.to_be_bytes()).ok(); // MEMORY_ACCESS_CTL
    lcd_dc.set_high().ok();
    lcd.write(&0x60_u8.to_be_bytes()).ok();

    // darw a blue square
    data[0] = 0x00;
    data[1] = 0x30;
    data[2] = 0x00;
    data[3] = 0xA0;
    lcd_dc.set_low().ok();
    lcd.write(&0x2A_u8.to_be_bytes()).ok(); // HORIZONTAL_ADDRESS_SET
    lcd_dc.set_high().ok();
    lcd.write(&data).ok();

    data[0] = 0x00;
    data[1] = 0x30;
    data[2] = 0x00;
    data[3] = 0xA0;
    lcd_dc.set_low().ok();
    lcd.write(&0x2b_u8.to_be_bytes()).ok(); // VERTICAL_ADDRESS_SET
    lcd_dc.set_high().ok();
    lcd.write(&data).ok();

    data[0] = 0xFF;
    data[1] = 0xE0;
    data[2] = 0xFF;
    data[3] = 0xE0;
    lcd_dc.set_low().ok();
    lcd.write(&0x2c_u8.to_be_bytes()).ok(); // MEMORY_WRITE
    lcd_dc.set_high().ok();
    for _ in 0..112 * 112 / 2 {
        lcd.write(&data).ok();
    }

    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        unsafe { riscv::asm::delay(100_000) }
    }
}
