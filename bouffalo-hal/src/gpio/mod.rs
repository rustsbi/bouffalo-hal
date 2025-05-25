//! General Purpose Input/Output.
//!
//! Generic Purpose Input/Output, or GPIO, is a standard interface used to connect the
//! microcontroller to external hardware. There are multiple GPIO pads on one chip,
//! which are different due to the signal of the internal connections. This structure
//! distinguishes them using constant generic number `N`.
//!
//! This structure represents a code ownership of Generic Purpose Input/Output pin.
//! GPIO pin structures are mutually exclusive, meaning that code in different contexts
//! are allowed use the same GPIO pin at the same time. Ownerships pass from structure
//! members to their wrappers; if one peripheral owns this pin, another peripheral
//! cannot use it at the same time.
//!
//! GPIO pin structure has an alternate mode parameter `M`. Alternate modes can be
//! switched by using functions begin with `into_`. Those functions internally operate
//! on hardware (specifically, GLB peripheral) to change pin function, and convert this
//! structure to `Pad` type with different alternate mode generic parameters. With new
//! alternate mode types, `Pad` structure would now match the demand of creating new
//! peripheral structures, or include specific functions for developers to use.
//!
//! # Examples
//!
//! A simple usage of GPIO pin is digital signal output.
//!
//! ```no_run
//! # use bouffalo_hal::gpio::{Pads, IntoPad};
//! # pub struct Peripherals { gpio: Pads<'static> }
//! # pub struct GLBv2;
//! # impl core::ops::Deref for GLBv2 {
//! #     type Target = bouffalo_hal::glb::RegisterBlock;
//! #     fn deref(&self) -> &Self::Target { unimplemented!() }
//! # }
//! # fn main() -> ! {
//! # let glb: &bouffalo_hal::glb::RegisterBlock = unsafe { &*core::ptr::null() };
//! # let p: Peripherals = Peripherals { gpio: Pads::__pads_from_glb(glb) };
//! use embedded_hal::digital::{OutputPin, PinState};
//!
//! // Switch io8 pin into floating output mode to prepare setting its state.
//! // It's connected to an LED to be controlled on hardware.
//! let mut led = p.gpio.io8.into_floating_output();
//! // Define a software LED state to be written into hardware.
//! let mut led_state = PinState::High;
//! loop {
//!     // We use function `set_state` to write GPIO electronic level.
//!     // This function is provided by floating output pin through embedded-hal
//!     // crate; we can also use `set_high` or `set_low` here.
//!     led.set_state(led_state).ok();
//!     // Now we flip software LED state to make the LED blink in next loop.
//!     led_state = !led_state;
//!     // Delay for some time for human eyes to discover the blink.
//!     for _ in 0..100_000 {
//!         unsafe { core::arch::asm!("nop") }
//!     }
//! }
//! # }
//! ```
//!
//! Peripheral structures usually limits the alternate mode of a GPIO pin. It must be
//! adjusted to the corresponding type before it can be used to create such structures.
//!
//! ```no_run
//! # use embedded_time::rate::*;
//! # use bouffalo_hal::{
//! #     clocks::Clocks,
//! #     gpio::{Pads, IntoPadv2},
//! #     uart::{BitOrder, Config, Parity, StopBits, WordLength},
//! # };
//! # use embedded_io::Write;
//! # pub struct Serial<PADS> { pads: PADS }
//! # impl<PADS> Serial<PADS> {
//! #     pub fn new<UART>(_: UART, _: Config, _: Baud,
//! # #[cfg(feature = "glb-v2")] _: PADS, _: &Clocks, _: &())
//! #     -> Self { unimplemented!() }
//! #     pub fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) -> Result<(), ()> { unimplemented!() }
//! #     pub fn flush(&mut self) -> Result<(), ()> { unimplemented!() }
//! # }
//! # pub struct Peripherals {
//! #     gpio: Pads<'static>,
//! #     glb: (),
//! #     uart0: UART0,
//! # }
//! # pub struct UART0;
//! # impl core::ops::Deref for UART0 {
//! #     type Target = bouffalo_hal::uart::RegisterBlock;
//! #     fn deref(&self) -> &Self::Target { unimplemented!() }
//! # }
//! # fn main() {
//! # let glb: &bouffalo_hal::glb::RegisterBlock = unsafe { &*core::ptr::null() };
//! # let p: Peripherals = Peripherals { gpio: Pads::__pads_from_glb(glb), glb: (), uart0: UART0 };
//! # let clocks = Clocks { xtal: Hertz(40_000_000) };
//! // Prepare UART transmit and receive pads by converting io14 and io15 into
//! // UART signal alternate mode.
//! # #[cfg(feature = "glb-v2")]
//! let tx = p.gpio.io14.into_uart();
//! # #[cfg(feature = "glb-v2")]
//! let rx = p.gpio.io15.into_uart();
//! # let sig2 = ();
//! # let sig3 = ();
//! # let config = Config::default();
//! // Create the serial structure. Note that if we don't have tx and rx GPIO
//! // alternate mode set correctly, code here won't compile for type mismatch.
//! # #[cfg(feature = "glb-v2")]
//! let mut serial = Serial::new(
//!     p.uart0,
//!     config,
//!     2000000.Bd(),
//!     ((tx, sig2), (rx, sig3)),
//!     &clocks,
//!     &p.glb,
//! );
//! # #[cfg(not(feature = "glb-v2"))]
//! # let mut serial = Serial { pads: () };
//! // Now that we have a working serial structure, we write something with it.
//! writeln!(serial, "Hello world!").ok();
//! serial.flush().ok();
//! # }
//! ```

mod alternate;
mod convert;
mod input;
mod output;
pub mod pad_v1;
pub mod pad_v2;
mod typestate;

use crate::glb::Version;
pub use convert::{IntoPad, IntoPadv2};
pub use typestate::*;

pub use {alternate::Alternate, input::Input, output::Output};

/// Peripheral instance of a GPIO pad.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a AnyRegisterBlock;
    /// Get the version of the corresponding register block.
    fn version(&self) -> Version;
}

/// GPIO pad instance with a number.
pub trait Numbered<'a, const N: usize>: Instance<'a> {}

// -- Any register block.
pub struct AnyRegisterBlock;

impl<'a> From<&'a crate::glb::v1::RegisterBlock> for &'a AnyRegisterBlock {
    #[inline]
    fn from(value: &'a crate::glb::v1::RegisterBlock) -> Self {
        unsafe { &*(value as *const _ as *const AnyRegisterBlock) }
    }
}

impl<'a> From<&'a crate::glb::v2::RegisterBlock> for &'a AnyRegisterBlock {
    #[inline]
    fn from(value: &'a crate::glb::v2::RegisterBlock) -> Self {
        unsafe { &*(value as *const _ as *const AnyRegisterBlock) }
    }
}

// -- Internal structures for pad structure with versions.

struct UnnumberedPad<'a>(&'a AnyRegisterBlock);
impl<'a> pad_v1::Instance<'a> for UnnumberedPad<'a> {
    #[inline]
    fn register_block(self) -> &'a crate::glb::v1::RegisterBlock {
        unsafe { &*(self.0 as *const _ as *const crate::glb::v1::RegisterBlock) }
    }
}
impl<'a> pad_v2::Instance<'a> for UnnumberedPad<'a> {
    #[inline]
    fn register_block(self) -> &'a crate::glb::v2::RegisterBlock {
        unsafe { &*(self.0 as *const _ as *const crate::glb::v2::RegisterBlock) }
    }
}

struct NumberedPad<'a, const N: usize>(&'a AnyRegisterBlock);
impl<'a, const N: usize> pad_v1::Instance<'a> for NumberedPad<'a, N> {
    #[inline]
    fn register_block(self) -> &'a crate::glb::v1::RegisterBlock {
        unsafe { &*(self.0 as *const _ as *const crate::glb::v1::RegisterBlock) }
    }
}
impl<'a, const N: usize> pad_v1::Numbered<'a, N> for NumberedPad<'a, N> {}
impl<'a, const N: usize> pad_v2::Instance<'a> for NumberedPad<'a, N> {
    #[inline]
    fn register_block(self) -> &'a crate::glb::v2::RegisterBlock {
        unsafe { &*(self.0 as *const _ as *const crate::glb::v2::RegisterBlock) }
    }
}
impl<'a, const N: usize> pad_v2::Numbered<'a, N> for NumberedPad<'a, N> {}
