use super::{
    alternate::Alternate,
    convert::{IntoPad, IntoPadv2},
    input::Input,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::RegisterBlock;
use core::ops::Deref;

/// GPIO pad which is disabled.
pub struct Disabled<GLB, const N: usize> {
    inner: super::Inner<GLB, N, typestate::Disabled>,
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
    fn into_spi<const I: usize>(self) -> Alternate<GLB, N, typestate::Spi<I>> {
        self.inner.into_spi().into()
    }
    #[inline]
    fn into_sdh(self) -> Alternate<GLB, N, typestate::Sdh> {
        self.inner.into_sdh().into()
    }
    #[inline]
    fn into_uart(self) -> Alternate<GLB, N, typestate::Uart> {
        self.inner.into_uart().into()
    }
    #[inline]
    fn into_mm_uart(self) -> Alternate<GLB, N, typestate::MmUart> {
        self.inner.into_mm_uart().into()
    }
    #[inline]
    fn into_pull_up_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_pull_up_pwm().into()
    }
    #[inline]
    fn into_pull_down_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_pull_down_pwm().into()
    }
    #[inline]
    fn into_floating_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>> {
        self.inner.into_floating_pwm().into()
    }
    #[inline]
    fn into_i2c<const I: usize>(self) -> Alternate<GLB, N, typestate::I2c<I>> {
        self.inner.into_i2c().into()
    }
    #[inline]
    fn into_jtag_d0(self) -> Alternate<GLB, N, typestate::JtagD0> {
        self.inner.into_jtag_d0().into()
    }
    #[inline]
    fn into_jtag_m0(self) -> Alternate<GLB, N, typestate::JtagM0> {
        self.inner.into_jtag_m0().into()
    }
    #[inline]
    fn into_jtag_lp(self) -> Alternate<GLB, N, typestate::JtagLp> {
        self.inner.into_jtag_lp().into()
    }
}

impl<GLB, const N: usize> From<super::Inner<GLB, N, typestate::Disabled>> for Disabled<GLB, N> {
    #[inline]
    fn from(inner: super::Inner<GLB, N, typestate::Disabled>) -> Self {
        Self { inner }
    }
}
