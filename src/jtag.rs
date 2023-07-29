//! JTAG interface feature support.

#[cfg(any(doc, feature = "glb-v2"))]
use crate::glb::v2::{Drive, Function, GpioConfig, Pull};
use crate::gpio::Alternate;
use crate::gpio::Pin;
use base_address::BaseAddress;
#[cfg(feature = "glb-v2")]
use core::marker::PhantomData;

/// D0 core JTAG mode (type state).
pub struct JtagD0;

/// M0 core JTAG mode (type state).
pub struct JtagM0;

/// LP core JTAG mode (type state).
pub struct JtagLp;

impl Alternate for JtagD0 {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagD0;
}

impl Alternate for JtagM0 {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagM0;
}

impl Alternate for JtagLp {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagLp;
}

// requires to set `.set_function(Function::JtagXx)` before use.
#[cfg(feature = "glb-v2")]
const JTAG_GPIO_CONFIG: GpioConfig = GpioConfig::RESET_VALUE
    .enable_input()
    .disable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::None);

impl<A: BaseAddress, const N: usize, M: Alternate> Pin<A, N, M> {
    /// Configures the pin to operate as D0 core JTAG.
    #[cfg(any(doc, feature = "glb-v2"))]
    #[inline]
    pub fn into_jtag_d0(self) -> Pin<A, N, JtagD0> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagD0);
        unsafe { self.base.gpio_config[N].write(config) };
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as M0 core JTAG.
    #[cfg(any(doc, feature = "glb-v2"))]
    #[inline]
    pub fn into_jtag_m0(self) -> Pin<A, N, JtagM0> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagM0);
        unsafe { self.base.gpio_config[N].write(config) };
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as LP core JTAG.
    #[cfg(any(doc, feature = "glb-v2"))]
    #[inline]
    pub fn into_jtag_lp(self) -> Pin<A, N, JtagLp> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagLp);
        unsafe { self.base.gpio_config[N].write(config) };
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
}
