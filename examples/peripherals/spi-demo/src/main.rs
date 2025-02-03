#![no_std]
#![no_main]

use bouffalo_hal::{prelude::*, spi::Spi};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_graphics::{
    draw_target::DrawTarget,
    image::*,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_hal::spi::MODE_0;
use mipidsi::Builder;
use mipidsi::{models::ST7789, options::ColorInversion};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;

    let spi_cs = p.gpio.io12.into_spi::<1>();
    let spi_mosi = p.gpio.io25.into_spi::<1>();
    let spi_clk = p.gpio.io19.into_spi::<1>();
    let lcd_dc = p.gpio.io13.into_floating_output();
    let mut lcd_bl = p.gpio.io11.into_floating_output();
    let lcd_rst = p.gpio.io24.into_floating_output();
    let spi_lcd = Spi::new(p.spi1, (spi_clk, spi_mosi, spi_cs), MODE_0, &p.glb);

    let mut delay = riscv::delay::McycleDelay::new(40_000_000);
    let di = display_interface_spi::SPIInterface::new(spi_lcd, lcd_dc);

    let mut display = Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(lcd_rst)
        .init(&mut delay)
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
        riscv::asm::delay(100_000);
    }
}
