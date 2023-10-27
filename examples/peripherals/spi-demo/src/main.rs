// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p spi-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{gpio::Pads, prelude::*, spi::Spi, GLB, SPI};
use embedded_graphics::{
    draw_target::DrawTarget,
    image::*,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_hal::spi::MODE_0;
use mipidsi::options::ColorInversion;
use mipidsi::Builder;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: Pads<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
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
    let lcd_dc = gpio.io13.into_floating_output();
    let mut lcd_bl = gpio.io11.into_floating_output();
    let lcd_rst = gpio.io24.into_floating_output();
    let spi_lcd = Spi::new(spi, (spi_clk, spi_mosi, spi_cs), MODE_0, &glb);

    let mut delay = riscv::delay::McycleDelay::new(40_000_000);
    let di = display_interface_spi::SPIInterfaceNoCS::new(spi_lcd, lcd_dc);

    let mut display = Builder::st7789(di)
        .with_invert_colors(ColorInversion::Inverted)
        .init(&mut delay, Some(lcd_rst))
        .unwrap();
    lcd_bl.set_high().ok();
    display.clear(Rgb565::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(0, 20));
    ferris.draw(&mut display).unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new("Hello World!", Point::new(10, 100), style)
        .draw(&mut display)
        .unwrap();

    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        unsafe { riscv::asm::delay(100_000) }
    }
}
