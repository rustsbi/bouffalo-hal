// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p pwm-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{
    clocks::Clocks,
    gpio::Pins,
    pwm::{ClockSource::Xclk, Pwm, SingleEnd},
    GLB, PWM,
};
use embedded_hal::{digital::OutputPin, pwm::SetDutyCycle};
use embedded_time::rate::units::Extensions;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let gpio: Pins<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let pwm: PWM<Static<0x2000A400>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {};

    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let led = gpio.io8.into_pull_down_pwm::<0>();

    let mut pwm = Pwm::new(pwm, SingleEnd, SingleEnd, &glb);
    pwm.group0.set_clock(1_000_000.Hz(), Xclk, &clocks);
    pwm.group0.set_max_duty_cycle(100);
    pwm.group0.start();
    let mut led = pwm.group0.channel0.positive_signal_pin(led);

    loop {
        for duty in 0..100 {
            led.set_duty_cycle(duty).ok();
            for _ in 0..1_000 {
                unsafe { core::arch::asm!("nop") }
            }
        }
        led.set_high().ok();
        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
        led.enable_pwm_output();
        for duty in (0..100).rev() {
            led.set_duty_cycle(duty).ok();
            for _ in 0..1_000 {
                unsafe { core::arch::asm!("nop") }
            }
        }
        led.set_low().ok();
        for _ in 0..200_000 {
            unsafe { core::arch::asm!("nop") }
        }
        led.enable_pwm_output();
    }
}
