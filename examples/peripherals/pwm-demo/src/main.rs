// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p pwm-demo

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Clocks, Peripherals};
use bl_soc::{
    prelude::*,
    pwm::{ClockSource::Xclk, Pwm, SingleEnd},
};
use embedded_time::rate::units::Extensions;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let led = p.gpio.io8.into_pull_down_pwm::<0>();

    let mut pwm = Pwm::new(p.pwm, SingleEnd, SingleEnd, &p.glb);
    pwm.group0.set_clock(1_000_000.Hz(), Xclk, &c);
    pwm.group0.set_max_duty_cycle(100);
    pwm.group0.start();
    let mut led = pwm.group0.channel0.positive_signal_pin(led);

    loop {
        for duty in 0..100 {
            led.set_duty_cycle(duty).ok();
            riscv::asm::delay(1_000);
        }
        led.set_high().ok();
        riscv::asm::delay(100_000);
        led.enable_pwm_output();
        for duty in (0..100).rev() {
            led.set_duty_cycle(duty).ok();
            riscv::asm::delay(1_000);
        }
        led.set_low().ok();
        riscv::asm::delay(200_000);
        led.enable_pwm_output();
    }
}
