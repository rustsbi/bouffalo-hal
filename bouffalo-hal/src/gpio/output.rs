use super::{Instance, input::Input, pad_v1::Outputv1, pad_v2::Outputv2};
use crate::glb::{Drive, Pull, Version};
use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};

/// Output mode driver for GPIO pad for all versions.
pub struct Output<'a> {
    number: usize,
    version: Version,
    // Register block pointer.
    base: &'a super::AnyRegisterBlock,
}

impl<'a> Output<'a> {
    /// Create an output driver for the GPIO pad.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Output<'a> {
        let version = gpio.version();
        let base = gpio.register_block();
        let (number, base) = match version {
            Version::V1 => Outputv1::new(super::UnnumberedPad(base), number, pull).into_inner(),
            Version::V2 => Outputv2::new(super::UnnumberedPad(base), number, pull).into_inner(),
        };
        Self {
            number,
            version,
            base,
        }
    }
}

impl<'a> Output<'a> {
    /// Get drive strength of this pad.
    #[inline]
    pub fn drive(&self) -> Drive {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).drive(),
            Version::V2 => Outputv2::at(self.number, self.base).drive(),
        }
    }
    /// Set drive strength of this pad.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_drive(val),
            Version::V2 => Outputv2::at(self.number, self.base).set_drive(val),
        }
    }
}

impl<'a> Output<'a> {
    /// Configures the pad to operate as a pull up output pad.
    #[inline]
    pub fn into_pull_up_output(self) -> Output<'a> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_pull(Pull::Up),
            Version::V2 => Outputv2::at(self.number, self.base).set_pull(Pull::Up),
        }
        self
    }
    /// Configures the pad to operate as a pull down output pad.
    #[inline]
    pub fn into_pull_down_output(self) -> Output<'a> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_pull(Pull::Down),
            Version::V2 => Outputv2::at(self.number, self.base).set_pull(Pull::Down),
        }
        self
    }
    /// Configures the pad to operate as a pull up input pad.
    #[inline]
    pub fn into_floating_output(self) -> Output<'a> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_pull(Pull::None),
            Version::V2 => Outputv2::at(self.number, self.base).set_pull(Pull::None),
        }
        self
    }
    /// Configures the pad to operate as a pull up input pad.
    #[inline]
    pub fn into_pull_up_input(self) -> Input<'a> {
        Input::new_from_inner(self.number, self.version, self.base, Pull::Up)
    }
    /// Configures the pad to operate as a pull down input pad.
    #[inline]
    pub fn into_pull_down_input(self) -> Input<'a> {
        Input::new_from_inner(self.number, self.version, self.base, Pull::Down)
    }
    /// Configures the pad to operate as a floating input pad.
    #[inline]
    pub fn into_floating_input(self) -> Input<'a> {
        Input::new_from_inner(self.number, self.version, self.base, Pull::None)
    }
}

impl<'a> ErrorType for Output<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for Output<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_low(),
            Version::V2 => Outputv2::at(self.number, self.base).set_low(),
        }
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).set_high(),
            Version::V2 => Outputv2::at(self.number, self.base).set_high(),
        }
    }
}

impl<'a> StatefulOutputPin for Output<'a> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).is_set_high(),
            Version::V2 => Outputv2::at(self.number, self.base).is_set_high(),
        }
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        match self.version {
            Version::V1 => Outputv1::at(self.number, self.base).is_set_low(),
            Version::V2 => Outputv2::at(self.number, self.base).is_set_low(),
        }
    }
}

// This part of implementation using `embedded_hal_027` is designed for backward compatibility of
// ecosystem crates, as some of them depends on embedded-hal v0.2.7 traits.
// We encourage ecosystem developers to use embedded-hal v1.0.0 traits; after that, this part of code
// would be removed in the future.
impl<'a> embedded_hal_027::digital::v2::OutputPin for Output<'a> {
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

impl<'a> Output<'a> {
    /// Internal constuctor, DO NOT USE.
    #[inline]
    pub(crate) fn new_from_inner(
        number: usize,
        version: Version,
        base: &'a super::AnyRegisterBlock,
        pull: Pull,
    ) -> Output<'a> {
        let (number, base) = match version {
            Version::V1 => Outputv1::new(super::UnnumberedPad(base), number, pull).into_inner(),
            Version::V2 => Outputv2::new(super::UnnumberedPad(base), number, pull).into_inner(),
        };
        Self {
            number,
            version,
            base,
        }
    }
}
