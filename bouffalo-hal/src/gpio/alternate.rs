use super::{
    IntoPadv2, Numbered,
    pad_v2::{self, Alternatev2},
    typestate::*,
};
use crate::glb::{Pull, Version};
use core::marker::PhantomData;

/// GPIO pad with alternate mode.
pub struct Alternate<'a, const N: usize, M> {
    version: Version,
    // Register block pointer.
    base: &'a super::AnyRegisterBlock,
    _mode: PhantomData<M>,
}

impl<'a, const N: usize> Alternate<'a, N, ()> {
    /// Create a UART signal driver from a GPIO pad instance.
    #[inline]
    pub fn new_uart(gpio: impl Numbered<'a, N>) -> Alternate<'a, N, Uart> {
        let version = gpio.version();
        let pad = super::NumberedPad::<N>(gpio.register_block());
        let base = match version {
            Version::V1 => todo!(),
            Version::V2 => Alternatev2::new_uart(pad).into_inner(),
        };
        Alternate {
            version,
            base,
            _mode: PhantomData,
        }
    }
    /// Create a Pulse Width Modulation signal driver from a GPIO pad instance.
    #[inline]
    pub fn new_pwm<const I: usize>(
        gpio: impl Numbered<'a, N>,
        pull: Pull,
    ) -> Alternate<'a, N, Pwm<I>> {
        let version = gpio.version();
        let pad = super::NumberedPad::<N>(gpio.register_block());
        let base = match version {
            Version::V1 => todo!(),
            Version::V2 => Alternatev2::new_pwm::<I>(pad, pull).into_inner(),
        };
        Alternate {
            version,
            base,
            _mode: PhantomData,
        }
    }
    /// Create an I2C signal driver from a GPIO pad instance.
    #[inline]
    pub fn new_i2c<const I: usize>(gpio: impl Numbered<'a, N>) -> Alternate<'a, N, I2c<I>> {
        let version = gpio.version();
        let pad = super::NumberedPad::<N>(gpio.register_block());
        let base = match version {
            Version::V1 => todo!(),
            Version::V2 => Alternatev2::new_i2c::<I>(pad).into_inner(),
        };
        Alternate {
            version,
            base,
            _mode: PhantomData,
        }
    }
    /// Create an SPI signal driver from a GPIO pad instance.
    #[inline]
    pub fn new_spi<const I: usize>(gpio: impl Numbered<'a, N>) -> Alternate<'a, N, Spi<I>> {
        let version = gpio.version();
        let pad = super::NumberedPad::<N>(gpio.register_block());
        let base = match version {
            Version::V1 => todo!(),
            Version::V2 => Alternatev2::new_spi::<I>(pad).into_inner(),
        };
        Alternate {
            version,
            base,
            _mode: PhantomData,
        }
    }
    /// Create an SPI signal driver from a GPIO pad instance.
    #[inline]
    pub fn new_sdh(gpio: impl Numbered<'a, N>) -> Alternate<'a, N, Sdh> {
        let version = gpio.version();
        let pad = super::NumberedPad::<N>(gpio.register_block());
        let base = match version {
            Version::V1 => todo!(),
            Version::V2 => Alternatev2::new_sdh(pad).into_inner(),
        };
        Alternate {
            version,
            base,
            _mode: PhantomData,
        }
    }
}

impl<'a, const N: usize> Alternate<'a, N, ()> {
    /// Create an input driver for the GPIO pad.
    #[inline]
    pub fn new_mm_uart(gpio: impl pad_v2::Numbered<'a, N>) -> Alternate<'a, N, MmUart> {
        let base = Alternatev2::new_mm_uart(gpio).into_inner();
        Alternate {
            version: Version::V2,
            base,
            _mode: PhantomData,
        }
    }
    /// Create a D0 core JTAG pad driver from a GPIO pad instance.
    #[inline]
    pub fn new_jtag_d0(gpio: impl pad_v2::Numbered<'a, N>) -> Alternate<'a, N, JtagD0> {
        let base = Alternatev2::new_jtag_d0(gpio).into_inner();
        Alternate {
            version: Version::V2,
            base,
            _mode: PhantomData,
        }
    }
    /// Create a M0 core JTAG pad driver from a GPIO pad instance.
    #[inline]
    pub fn new_jtag_m0(gpio: impl pad_v2::Numbered<'a, N>) -> Alternate<'a, N, JtagM0> {
        let base = Alternatev2::new_jtag_m0(gpio).into_inner();
        Alternate {
            version: Version::V2,
            base,
            _mode: PhantomData,
        }
    }
    /// Create a LP core JTAG pad driver from a GPIO pad instance.
    #[inline]
    pub fn new_jtag_lp(gpio: impl pad_v2::Numbered<'a, N>) -> Alternate<'a, N, JtagLp> {
        let base = Alternatev2::new_jtag_lp(gpio).into_inner();
        Alternate {
            version: Version::V2,
            base,
            _mode: PhantomData,
        }
    }
}

impl<'a, const N: usize, M> IntoPadv2<'a, N> for Alternate<'a, N, M> {
    #[inline]
    fn into_spi<const I: usize>(self) -> Alternate<'a, N, super::typestate::Spi<I>> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_spi::<I>(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_sdh(self) -> Alternate<'a, N, super::typestate::Sdh> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_sdh(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_uart(self) -> Alternate<'a, N, super::typestate::Uart> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_uart(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_mm_uart(self) -> Alternate<'a, N, super::typestate::MmUart> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_mm_uart(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_pull_up_pwm<const I: usize>(self) -> Alternate<'a, N, super::typestate::Pwm<I>> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_pwm::<I>(super::NumberedPad::<N>(self.base), Pull::Up),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_pull_down_pwm<const I: usize>(self) -> Alternate<'a, N, super::typestate::Pwm<I>> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => {
                Alternatev2::new_pwm::<I>(super::NumberedPad::<N>(self.base), Pull::Down)
            }
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_floating_pwm<const I: usize>(self) -> Alternate<'a, N, super::typestate::Pwm<I>> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => {
                Alternatev2::new_pwm::<I>(super::NumberedPad::<N>(self.base), Pull::None)
            }
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_i2c<const I: usize>(self) -> Alternate<'a, N, super::typestate::I2c<I>> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_i2c::<I>(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_jtag_d0(self) -> Alternate<'a, N, super::typestate::JtagD0> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_jtag_d0(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_jtag_m0(self) -> Alternate<'a, N, super::typestate::JtagM0> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_jtag_m0(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    fn into_jtag_lp(self) -> Alternate<'a, N, super::typestate::JtagLp> {
        match self.version {
            Version::V1 => fail_only_version_2(),
            Version::V2 => Alternatev2::new_jtag_lp(super::NumberedPad::<N>(self.base)),
        };
        Alternate {
            version: self.version,
            base: self.base,
            _mode: PhantomData,
        }
    }
}

#[cold]
fn fail_only_version_2() -> ! {
    unimplemented!("only version 2 alternate supports IntoPadv2 conversation")
}
