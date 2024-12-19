use super::{
    convert::{IntoPad, IntoPadv2},
    input::Input,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::RegisterBlock;
use core::ops::Deref;

/// GPIO pad which is disabled.
pub struct Disabled<GLB, const N: usize> {
    #[cfg(feature = "glb-v1")]
    inner: super::pad_v1::Padv1<GLB, N, typestate::Disabled>,
    #[cfg(feature = "glb-v2")]
    inner: super::pad_v2::Padv2<GLB, N, typestate::Disabled>,
    #[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
    inner: super::pad_dummy::PadDummy<GLB, N, typestate::Disabled>,
}

impl<GLB: Deref<Target = RegisterBlock>, const N: usize> IntoPad<GLB, N> for Disabled<GLB, N> {
    #[inline]
    fn into_pull_up_output(self) -> Output<GLB, N, PullUp> {
        self.inner.into_pull_up_output().into()
    }
    #[inline]
    fn into_pull_down_output(self) -> Output<GLB, N, PullDown> {
        self.inner.into_pull_down_output().into()
    }
    #[inline]
    fn into_floating_output(self) -> Output<GLB, N, Floating> {
        self.inner.into_floating_output().into()
    }
    #[inline]
    fn into_pull_up_input(self) -> Input<GLB, N, PullUp> {
        self.inner.into_pull_up_input().into()
    }
    #[inline]
    fn into_pull_down_input(self) -> Input<GLB, N, PullDown> {
        self.inner.into_pull_down_input().into()
    }
    #[inline]
    fn into_floating_input(self) -> Input<GLB, N, Floating> {
        self.inner.into_floating_input().into()
    }
}

#[cfg(any(doc, feature = "glb-v2"))]
impl<GLB: Deref<Target = RegisterBlock>, const N: usize> IntoPadv2<GLB, N> for Disabled<GLB, N> {
    #[inline]
    fn into_spi<const I: usize>(self) -> super::Pad<GLB, N, super::typestate::Spi<I>> {
        super::Pad {
            inner: self.inner.into_spi(),
        }
    }
    #[inline]
    fn into_sdh(self) -> super::Pad<GLB, N, super::typestate::Sdh> {
        super::Pad {
            inner: self.inner.into_sdh(),
        }
    }
    #[inline]
    fn into_uart(self) -> super::Pad<GLB, N, super::typestate::Uart> {
        super::Pad {
            inner: self.inner.into_uart(),
        }
    }
    #[inline]
    fn into_mm_uart(self) -> super::Pad<GLB, N, super::typestate::MmUart> {
        super::Pad {
            inner: self.inner.into_mm_uart(),
        }
    }
    #[inline]
    fn into_pull_up_pwm<const I: usize>(self) -> super::Pad<GLB, N, super::typestate::Pwm<I>> {
        super::Pad {
            inner: self.inner.into_pull_up_pwm(),
        }
    }
    #[inline]
    fn into_pull_down_pwm<const I: usize>(self) -> super::Pad<GLB, N, super::typestate::Pwm<I>> {
        super::Pad {
            inner: self.inner.into_pull_down_pwm(),
        }
    }
    #[inline]
    fn into_floating_pwm<const I: usize>(self) -> super::Pad<GLB, N, super::typestate::Pwm<I>> {
        super::Pad {
            inner: self.inner.into_floating_pwm(),
        }
    }
    #[inline]
    fn into_i2c<const I: usize>(self) -> super::Pad<GLB, N, super::typestate::I2c<I>> {
        super::Pad {
            inner: self.inner.into_i2c(),
        }
    }
    #[inline]
    fn into_jtag_d0(self) -> super::Pad<GLB, N, super::typestate::JtagD0> {
        super::Pad {
            inner: self.inner.into_jtag_d0(),
        }
    }
    #[inline]
    fn into_jtag_m0(self) -> super::Pad<GLB, N, super::typestate::JtagM0> {
        super::Pad {
            inner: self.inner.into_jtag_m0(),
        }
    }
    #[inline]
    fn into_jtag_lp(self) -> super::Pad<GLB, N, super::typestate::JtagLp> {
        super::Pad {
            inner: self.inner.into_jtag_lp(),
        }
    }
}
