use super::{Instance, Output, pad_v1::Inputv1, pad_v2::Inputv2};
use crate::glb::{Pull, Version};
use embedded_hal::digital::{ErrorType, InputPin};

/// Input mode driver for GPIO pad for all versions.
pub struct Input<'a> {
    number: usize,
    version: Version,
    // Register block pointer.
    base: &'a super::AnyRegisterBlock,
}

impl<'a> Input<'a> {
    /// Create an input driver for the GPIO pad.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Input<'a> {
        let version = gpio.version();
        let base = gpio.register_block();
        let (number, base) = match version {
            Version::V1 => Inputv1::new(super::UnnumberedPad(base), number, pull).into_inner(),
            Version::V2 => Inputv2::new(super::UnnumberedPad(base), number, pull).into_inner(),
        };
        Self {
            number,
            version,
            base,
        }
    }
}

impl<'a> Input<'a> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).enable_schmitt(),
            Version::V2 => Inputv2::at(self.number, self.base).enable_schmitt(),
        }
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).disable_schmitt(),
            Version::V2 => Inputv2::at(self.number, self.base).disable_schmitt(),
        }
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).clear_interrupt(),
            Version::V2 => Inputv2::at(self.number, self.base).clear_interrupt(),
        }
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).has_interrupt(),
            Version::V2 => Inputv2::at(self.number, self.base).has_interrupt(),
        }
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).mask_interrupt(),
            Version::V2 => Inputv2::at(self.number, self.base).mask_interrupt(),
        }
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).unmask_interrupt(),
            Version::V2 => Inputv2::at(self.number, self.base).unmask_interrupt(),
        }
    }
}

impl<'a> Input<'a> {
    /// Configures the pad to operate as a pull up output pad.
    #[inline]
    pub fn into_pull_up_output(self) -> Output<'a> {
        Output::new_from_inner(self.number, self.version, self.base, Pull::Up)
    }
    /// Configures the pad to operate as a pull down output pad.
    #[inline]
    pub fn into_pull_down_output(self) -> Output<'a> {
        Output::new_from_inner(self.number, self.version, self.base, Pull::Down)
    }
    /// Configures the pad to operate as a pull up input pad.
    #[inline]
    pub fn into_floating_output(self) -> Output<'a> {
        Output::new_from_inner(self.number, self.version, self.base, Pull::None)
    }
    #[inline]
    pub fn into_pull_up_input(self) -> Input<'a> {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).set_pull(Pull::Up),
            Version::V2 => Inputv2::at(self.number, self.base).set_pull(Pull::Up),
        }
        self
    }
    /// Configures the pad to operate as a pull down input pad.
    #[inline]
    pub fn into_pull_down_input(self) -> Input<'a> {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).set_pull(Pull::Down),
            Version::V2 => Inputv2::at(self.number, self.base).set_pull(Pull::Down),
        }
        self
    }
    /// Configures the pad to operate as a floating input pad.
    #[inline]
    pub fn into_floating_input(self) -> Input<'a> {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).set_pull(Pull::None),
            Version::V2 => Inputv2::at(self.number, self.base).set_pull(Pull::None),
        }
        self
    }
}

impl<'a> ErrorType for Input<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> InputPin for Input<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).is_high(),
            Version::V2 => Inputv2::at(self.number, self.base).is_high(),
        }
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        match self.version {
            Version::V1 => Inputv1::at(self.number, self.base).is_low(),
            Version::V2 => Inputv2::at(self.number, self.base).is_low(),
        }
    }
}

impl<'a> Input<'a> {
    /// Internal constuctor, DO NOT USE.
    #[inline]
    pub(crate) fn new_from_inner(
        number: usize,
        version: Version,
        base: &'a super::AnyRegisterBlock,
        pull: Pull,
    ) -> Input<'a> {
        let (number, base) = match version {
            Version::V1 => Inputv1::new(super::UnnumberedPad(base), number, pull).into_inner(),
            Version::V2 => Inputv2::new(super::UnnumberedPad(base), number, pull).into_inner(),
        };
        Self {
            number,
            version,
            base,
        }
    }
}
