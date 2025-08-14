#![no_std]
#![no_main]

use bouffalo_hal::{i2c::I2c, prelude::*, spi::Spi, uart::Config};
use bouffalo_rt::{Clocks, Peripherals, entry};
use core::fmt::Write;
use cst816s::{CST816S, TouchGesture};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::{
    draw_target::DrawTarget,
    image::*,
    mono_font::{MonoTextStyle, ascii::*},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
    text::renderer::CharacterStyle,
};
use embedded_hal::spi::MODE_0;
use embedded_time::rate::*;
use mipidsi::{Builder, interface::SpiInterface, models::ST7789, options::ColorInversion};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();
    let mut led = p.gpio.io8.into_floating_output();

    let spi_cs = p.gpio.io12;
    let spi_mosi = p.gpio.io25;
    let spi_clk = p.gpio.io19;
    let lcd_dc = p.gpio.io13.into_floating_output();
    let mut lcd_bl = p.gpio.io11.into_floating_output();
    let lcd_rst = p.gpio.io24.into_floating_output();
    let spi_lcd = Spi::transmit_only(p.spi1, (spi_clk, spi_mosi, spi_cs), MODE_0, &p.glb);

    // Touch panel need reset, but the reset pin is shared with the LCD, so use fake pin
    let scl = p.gpio.io6;
    let sda = p.gpio.io7;
    let touch_int = p.gpio.io32.into_pull_up_input();
    // let touch_rst = p.gpio.io24.into_floating_output();
    let fake_touch_rst = p.gpio.io26.into_floating_output();
    let i2c = I2c::new(p.i2c0, (scl, sda), 100_000.Hz(), &p.glb);

    let mut delay = riscv::delay::McycleDelay::new(40_000_000);
    let mut buffer = [0u8; 512];
    let di = SpiInterface::new(spi_lcd, lcd_dc, &mut buffer);

    let mut display = Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(lcd_rst)
        .init(&mut delay)
        .unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    lcd_bl.set_high().ok();
    let mut touch = CST816S::new(i2c, touch_int, fake_touch_rst);

    led.set_high().ok();

    touch.setup(&mut delay).ok();

    writeln!(serial, "Hello Rust🦀!").ok();
    writeln!(
        serial,
        "Welcome to I2C screen demo, slide the screen to set ferris position"
    )
    .ok();

    let mut style = MonoTextStyle::new(&FONT_9X15, Rgb565::WHITE);
    style.set_background_color(Some(Rgb565::BLACK));

    Text::new(
        "Slide the screen to \nset ferris position\nLongpress to center",
        Point::new(20, 35),
        style,
    )
    .draw(&mut display)
    .unwrap();

    let mut tmp = [0u8; 64];
    let mut sb = StrBuf {
        buf: &mut tmp,
        len: 0,
    };
    const COLS: usize = 20;
    let raw_image_data = ImageRawLE::new(include_bytes!("ferris.raw"), 86);

    // Display size and reserved text area
    let disp_size = display.size();
    let disp_w = disp_size.width as i32;
    let disp_h = disp_size.height as i32;
    const TEXT_AREA_H: i32 = 110; // Top reserved area for text; adjust as needed

    // Probe the image once to get width/height
    let img_probe = Image::new(&raw_image_data, Point::new(0, 0));
    let img_w = img_probe.bounding_box().size.width as i32;
    let img_h = img_probe.bounding_box().size.height as i32;

    // Track last image bounding box to erase
    let mut last_bbox: Option<Rectangle> = None;
    let erase_style = PrimitiveStyle::with_fill(Rgb565::BLACK);

    loop {
        if let Some(event) = touch.read_one_touch_event(true) {
            writeln!(serial, "Touch event: {:?}", event).ok();

            // Format two lines with fixed-width columns
            sb.clear();
            let _ = write!(&mut sb, "Point: ({}, {})", event.x, event.y);
            sb.pad_line_to(COLS);
            sb.newline();
            let _ = write!(&mut sb, "Gesture: {:?}", event.gesture);
            sb.pad_line_to(COLS);

            let text = core::str::from_utf8(&sb.buf[..sb.len]).unwrap_or("");
            Text::new(text, Point::new(20, 80), style)
                .draw(&mut display)
                .unwrap();

            // Compute new position (keep out of text area)
            let avail_h = disp_h - TEXT_AREA_H;
            let center_x = (disp_w - img_w).max(0) / 2;
            let center_y = TEXT_AREA_H + (avail_h - img_h).max(0) / 2;
            let top_y = TEXT_AREA_H;
            // Black border adjustment for the screen
            let bottom_y = (disp_h - img_h - 20).max(TEXT_AREA_H);
            let mid_y = center_y;

            let new_pos = match event.gesture {
                TouchGesture::SlideUp => Point::new(center_x, top_y),
                TouchGesture::SlideDown => Point::new(center_x, bottom_y),
                TouchGesture::SlideLeft => Point::new(0, mid_y),
                TouchGesture::SlideRight => Point::new(disp_w - img_w, mid_y),
                TouchGesture::LongPress => Point::new(center_x, center_y),
                _ => {
                    // Ignore other gestures
                    led.set_low().ok();
                    continue;
                }
            };

            // Erase last image area, then draw at new position
            if let Some(bb) = last_bbox {
                bb.into_styled(erase_style).draw(&mut display).ok();
            }
            let ferris = Image::new(&raw_image_data, new_pos);
            ferris.draw(&mut display).unwrap();
            last_bbox = Some(ferris.bounding_box());

            led.set_low().ok();
        } else {
            led.set_high().ok();
        }
    }
}

struct StrBuf<'a> {
    buf: &'a mut [u8],
    len: usize,
}
// Impl core::fmt::Write for StrBuf
impl<'a> core::fmt::Write for StrBuf<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let space = self.buf.len().saturating_sub(self.len);
        let n = core::cmp::min(space, bytes.len());
        self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
        self.len += n;
        Ok(())
    }
}

impl<'a> StrBuf<'a> {
    // Reset the buffer (fill with zeros is optional; we keep it for clarity)
    pub fn clear(&mut self) {
        self.buf.fill(0);
        self.len = 0;
    }

    // Current line length since last '\n'
    pub fn line_len(&self) -> usize {
        let mut i = self.len;
        while i > 0 {
            if self.buf[i - 1] == b'\n' {
                break;
            }
            i -= 1;
        }
        self.len - i
    }

    // Push a single byte if capacity allows
    fn push(&mut self, b: u8) {
        if self.len < self.buf.len() {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }

    // Append a newline
    pub fn newline(&mut self) {
        self.push(b'\n');
    }

    // Pad the current line with spaces to a fixed column width
    pub fn pad_line_to(&mut self, cols: usize) {
        let mut cur = self.line_len();
        while cur < cols {
            self.push(b' ');
            cur += 1;
        }
    }
}
