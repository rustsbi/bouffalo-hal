//! General Purpose Input/Output.

mod convert;
mod disabled;
mod gpio_group;
mod input;
mod output;
mod pad_dummy;
mod pad_v1;
mod pad_v2;
mod typestate;

pub use convert::IntoPad;
pub use disabled::Disabled as DisabledPad;
pub use gpio_group::Pads;
pub use input::Input as InputPad;
pub use output::Output as OutputPad;
pub use typestate::*;
pub use {pad_v1::Padv1, pad_v2::Padv2};

use crate::glb::{self, Drive};
use core::ops::Deref;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

cfg_if::cfg_if! {
    if #[cfg(feature = "glb-v1")] {
        pub use crate::glb::v1;
    } else if #[cfg(feature = "glb-v2")] {
        pub use crate::glb::v2;
    }
}

/// Individual GPIO pad.
///
/// Generic Purpose Input/Output, or GPIO, is a standard interface used to connect the
/// microcontroller to external hardware. There are multiple GPIO pads on one chip,
/// which are different due to the signal of the internal connections. This structure
/// distinguishes them using constant generic number `N`.
///
/// This structure represents a code ownership of Generic Purpose Input/Output pin.
/// GPIO pin structures are mutually exclusive, meaning that code in different contexts
/// are allowed use the same GPIO pin at the same time. Ownerships pass from structure
/// members to their wrappers; if one peripheral owns this pin, another peripheral
/// cannot use it at the same time.
///
/// GPIO pin structure has an alternate mode parameter `M`. Alternate modes can be
/// switched by using functions begin with `into_`. Those functions internally operate
/// on hardware (specifically, GLB peripheral) to change pin function, and convert this
/// structure to `Pin` type with different alternate mode generic parameters. With new
/// alternate mode types, `Pin` structure would now match the demand of creating new
/// peripheral structures, or include specific functions for developers to use.
///
/// # Examples
///
/// A simple usage of GPIO pin is digital signal output.
///
/// ```no_run
/// # use bouffalo_hal::gpio::Pads;
/// # pub struct Peripherals { gpio: Pads<GLBv2> }
/// # pub struct GLBv2;
/// # impl core::ops::Deref for GLBv2 {
/// #     type Target = bouffalo_hal::glb::RegisterBlock;
/// #     fn deref(&self) -> &Self::Target { unimplemented!() }
/// # }
/// # fn main() -> ! {
/// #   let p: Peripherals = unsafe { core::mem::transmute(()) };
/// use embedded_hal::digital::{OutputPin, PinState};
///
/// // Switch io8 pin into floating output mode to prepare setting its state.
/// // It's connected to an LED to be controlled on hardware.
/// let mut led = p.gpio.io8.into_floating_output();
/// // Define a software LED state to be written into hardware.
/// let mut led_state = PinState::High;
/// loop {
///     // We use function `set_state` to write GPIO electronic level.
///     // This function is provided by floating output pin through embedded-hal
///     // crate; we can also use `set_high` or `set_low` here.
///     led.set_state(led_state).ok();
///     // Now we flip software LED state to make the LED blink in next loop.
///     led_state = !led_state;
///     // Delay for some time for human eyes to discover the blink.
///     for _ in 0..100_000 {
///         unsafe { core::arch::asm!("nop") }
///     }
/// }
/// # }
/// ```
///
/// Peripheral structures usually limits the alternate mode of a GPIO pin. It must be
/// adjusted to the corresponding type before it can be used to create such structures.
///
/// ```no_run
/// # use embedded_time::rate::*;
/// # use bouffalo_hal::{
/// #     clocks::Clocks,
/// #     gpio::{Pads, Pad, Alternate},
/// #     uart::{BitOrder, Config, Parity, StopBits, WordLength},
/// # };
/// # use embedded_io::Write;
/// # pub struct Serial<PADS> { pads: PADS }
/// # impl<PADS> Serial<PADS> {
/// #     pub fn new<UART>(_: UART, _: Config, _: Baud,
/// # #[cfg(feature = "glb-v2")] _: PADS, _: &Clocks, _: &GLBv2)
/// #     -> Self { unimplemented!() }
/// #     pub fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) -> Result<(), ()> { unimplemented!() }
/// #     pub fn flush(&mut self) -> Result<(), ()> { unimplemented!() }
/// # }
/// # pub struct Peripherals {
/// #     gpio: Pads<GLBv2>,
/// #     glb: GLBv2,
/// #     uart0: UART0,
/// # }
/// # pub struct GLBv2;
/// # impl core::ops::Deref for GLBv2 {
/// #     type Target = bouffalo_hal::glb::RegisterBlock;
/// #     fn deref(&self) -> &Self::Target { unimplemented!() }
/// # }
/// # pub struct UART0;
/// # impl core::ops::Deref for UART0 {
/// #     type Target = bouffalo_hal::uart::RegisterBlock;
/// #     fn deref(&self) -> &Self::Target { unimplemented!() }
/// # }
/// # fn main() {
/// # let p: Peripherals = unsafe { core::mem::transmute(()) };
/// # let clocks = Clocks { xtal: Hertz(40_000_000) };
/// // Prepare UART transmit and receive pads by converting io14 and io15 into
/// // UART signal alternate mode.
/// # #[cfg(feature = "glb-v2")]
/// let tx = p.gpio.io14.into_uart();
/// # #[cfg(feature = "glb-v2")]
/// let rx = p.gpio.io15.into_uart();
/// # let sig2 = ();
/// # let sig3 = ();
/// # let config = Config::default();
/// // Create the serial structure. Note that if we don't have tx and rx GPIO
/// // alternate mode set correctly, code here won't compile for type mismatch.
/// # #[cfg(feature = "glb-v2")]
/// let mut serial = Serial::new(
///     p.uart0,
///     config,
///     2000000.Bd(),
///     ((tx, sig2), (rx, sig3)),
///     &clocks,
///     &p.glb,
/// );
/// # #[cfg(not(feature = "glb-v2"))]
/// # let mut serial = Serial { pads: () };
/// // Now that we have a working serial structure, we write something with it.
/// writeln!(serial, "Hello world!").ok();
/// serial.flush().ok();
/// # }
/// ```
pub struct Pad<GLB, const N: usize, M> {
    #[cfg(feature = "glb-v1")]
    inner: pad_v1::Padv1<GLB, N, M>,
    #[cfg(feature = "glb-v2")]
    inner: pad_v2::Padv2<GLB, N, M>,
    #[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
    inner: pad_dummy::PadDummy<GLB, N, M>,
}

impl<GLB, const N: usize, M> ErrorType for Pad<GLB, N, Input<M>> {
    type Error = core::convert::Infallible;
}

impl<GLB, const N: usize, M> ErrorType for Pad<GLB, N, Output<M>> {
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> InputPin
    for Pad<GLB, N, Input<M>>
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_high()
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_low()
    }
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> OutputPin
    for Pad<GLB, N, Output<M>>
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.inner.set_low()
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.inner.set_high()
    }
}

// This part of implementation using `embedded_hal_027` is designed for backward compatibility of
// ecosystem crates, as some of them depends on embedded-hal v0.2.7 traits.
// We encourage ecosystem developers to use embedded-hal v1.0.0 traits; after that, this part of code
// would be removed in the future.
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M>
    embedded_hal_027::digital::v2::OutputPin for Pad<GLB, N, Output<M>>
{
    type Error = core::convert::Infallible;
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        <Self as OutputPin>::set_low(self)
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        <Self as OutputPin>::set_high(self)
    }
}

// We do not support StatefulOutputPin in embedded-hal 1.0.0 here, because the hardware does not
// have such functionality to read back the previously set pin state.
// It is recommended that users add a variable to store the pin state if necessary; see examples/gpio-demo.

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, Input<M>> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        self.inner.enable_schmitt()
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        self.inner.disable_schmitt()
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.inner.clear_interrupt()
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        self.inner.has_interrupt()
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        self.inner.mask_interrupt()
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        self.inner.unmask_interrupt();
    }
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, Output<M>> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.inner.drive()
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        self.inner.set_drive(val)
    }
}

#[cfg(feature = "glb-v1")]
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, Input<M>> {
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> v1::InterruptMode {
        self.inner.interrupt_mode()
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: v1::InterruptMode) {
        self.inner.set_interrupt_mode(val)
    }
}

#[cfg(feature = "glb-v2")]
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, Input<M>> {
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> v2::InterruptMode {
        self.inner.interrupt_mode()
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: v2::InterruptMode) {
        self.inner.set_interrupt_mode(val)
    }
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, M> {
    /// Configures the pin to operate as a pull up output pin.
    #[inline]
    pub fn into_pull_up_output(self) -> Pad<GLB, N, Output<PullUp>> {
        Pad {
            inner: self.inner.into_pull_up_output(),
        }
    }
    /// Configures the pin to operate as a pull down output pin.
    #[inline]
    pub fn into_pull_down_output(self) -> Pad<GLB, N, Output<PullDown>> {
        Pad {
            inner: self.inner.into_pull_down_output(),
        }
    }
    /// Configures the pin to operate as a floating output pin.
    #[inline]
    pub fn into_floating_output(self) -> Pad<GLB, N, Output<Floating>> {
        Pad {
            inner: self.inner.into_floating_output(),
        }
    }
    /// Configures the pin to operate as a pull up input pin.
    #[inline]
    pub fn into_pull_up_input(self) -> Pad<GLB, N, Input<PullUp>> {
        Pad {
            inner: self.inner.into_pull_up_input(),
        }
    }
    /// Configures the pin to operate as a pull down input pin.
    #[inline]
    pub fn into_pull_down_input(self) -> Pad<GLB, N, Input<PullDown>> {
        Pad {
            inner: self.inner.into_pull_down_input(),
        }
    }
    /// Configures the pin to operate as a floating input pin.
    #[inline]
    pub fn into_floating_input(self) -> Pad<GLB, N, Input<Floating>> {
        Pad {
            inner: self.inner.into_floating_input(),
        }
    }
}

// TODO: #[doc(cfg(feature = "glb-v2"))] once feature(doc_cfg) is stablized
#[cfg(any(doc, feature = "glb-v2"))]
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Pad<GLB, N, M> {
    /// Configures the pin to operate as a SPI pin.
    #[inline]
    pub fn into_spi<const I: usize>(self) -> Pad<GLB, N, Spi<I>>
    where
        Spi<I>: Alternate,
    {
        Pad {
            inner: self.inner.into_spi(),
        }
    }
    /// Configures the pin to operate as a SDH pin.
    #[inline]
    pub fn into_sdh(self) -> Pad<GLB, N, Sdh> {
        Pad {
            inner: self.inner.into_sdh(),
        }
    }
    /// Configures the pin to operate as UART signal.
    #[inline]
    pub fn into_uart(self) -> Pad<GLB, N, Uart> {
        Pad {
            inner: self.inner.into_uart(),
        }
    }
    /// Configures the pin to operate as multi-media cluster UART signal.
    #[inline]
    pub fn into_mm_uart(self) -> Pad<GLB, N, MmUart> {
        Pad {
            inner: self.inner.into_mm_uart(),
        }
    }
    /// Configures the pin to operate as a pull up Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_pull_up_pwm<const I: usize>(self) -> Pad<GLB, N, Pwm<I>>
    where
        Pwm<I>: Alternate,
    {
        Pad {
            inner: self.inner.into_pull_up_pwm(),
        }
    }
    /// Configures the pin to operate as a pull down Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_pull_down_pwm<const I: usize>(self) -> Pad<GLB, N, Pwm<I>>
    where
        Pwm<I>: Alternate,
    {
        Pad {
            inner: self.inner.into_pull_down_pwm(),
        }
    }
    /// Configures the pin to operate as floating Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_floating_pwm<const I: usize>(self) -> Pad<GLB, N, Pwm<I>>
    where
        Pwm<I>: Alternate,
    {
        Pad {
            inner: self.inner.into_floating_pwm(),
        }
    }
    /// Configures the pin to operate as an Inter-Integrated Circuit signal pin.
    #[inline]
    pub fn into_i2c<const I: usize>(self) -> Pad<GLB, N, I2c<I>>
    where
        I2c<I>: Alternate,
    {
        Pad {
            inner: self.inner.into_i2c(),
        }
    }
    /// Configures the pin to operate as D0 core JTAG.
    #[inline]
    pub fn into_jtag_d0(self) -> Pad<GLB, N, JtagD0> {
        Pad {
            inner: self.inner.into_jtag_d0(),
        }
    }
    /// Configures the pin to operate as M0 core JTAG.
    #[inline]
    pub fn into_jtag_m0(self) -> Pad<GLB, N, JtagM0> {
        Pad {
            inner: self.inner.into_jtag_m0(),
        }
    }
    /// Configures the pin to operate as LP core JTAG.
    #[inline]
    pub fn into_jtag_lp(self) -> Pad<GLB, N, JtagLp> {
        Pad {
            inner: self.inner.into_jtag_lp(),
        }
    }
}
