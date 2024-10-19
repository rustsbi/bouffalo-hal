#![no_std]
#![no_main]

use bouffalo_hal::prelude::*;
use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_cli::{cli::CliBuilder, Command};
use embedded_time::rate::*;
use panic_halt as _;

#[derive(Command)]
enum Base {
    /// Print out 'Hello world!'.
    Hello,
    /// LED control command.
    Led {
        #[command(subcommand)]
        command: Option<LedCommand>,
    },
}

#[derive(Command)]
enum LedCommand {
    /// Turn on LED.
    On,
    /// Turn off LED.
    Off,
    /// Switch LED state.
    Switch,
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let serial = p
        .uart0
        .freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &c);

    let (mut tx, mut rx) = serial.split();

    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;

    writeln!(tx, "Welcome to embedded-cli example by bouffalo-hal🦀!").ok();
    writeln!(tx, "For command helps, type 'help'.").ok();

    let (command_buffer, history_buffer) = ([0; 128], [0; 128]);
    let mut cli = CliBuilder::default()
        .writer(tx)
        .command_buffer(command_buffer)
        .history_buffer(history_buffer)
        .prompt("uart-cli-demo> ")
        .build()
        .unwrap();

    loop {
        led.set_state(led_state).ok();
        let mut slice = [0];
        rx.read_exact(&mut slice).ok();
        let _ = cli.process_byte::<Base, _>(
            slice[0],
            &mut Base::processor(|cli, command| {
                match command {
                    Base::Hello => {
                        cli.writer().write_str("Hello world!").ok();
                    }
                    Base::Led { command } => match command {
                        Some(LedCommand::On) => led_state = PinState::Low,
                        Some(LedCommand::Off) => led_state = PinState::High,
                        Some(LedCommand::Switch) => led_state = !led_state,
                        None => match led_state {
                            PinState::High => cli.writer().write_str("LED state: High").unwrap(),
                            PinState::Low => cli.writer().write_str("LED state: Low").unwrap(),
                        },
                    },
                }
                Ok(())
            }),
        );
    }
}
