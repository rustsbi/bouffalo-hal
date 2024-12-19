#[cfg(any(doc, feature = "glb-v2"))]
use super::{alternate::Alternate, convert::IntoPadv2};
use super::{
    convert::IntoPad,
    input::Input,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};

/// GPIO pad which is disabled.
pub struct Disabled<'a, const N: usize> {
    inner: super::Inner<'a, N, typestate::Disabled>,
}

impl<'a, const N: usize> IntoPad<'a, N> for Disabled<'a, N> {
    #[inline]
    fn into_pull_up_output(self) -> Output<'a, N, PullUp> {
        self.inner.into_pull_up_output().into()
    }
    #[inline]
    fn into_pull_down_output(self) -> Output<'a, N, PullDown> {
        self.inner.into_pull_down_output().into()
    }
    #[inline]
    fn into_floating_output(self) -> Output<'a, N, Floating> {
        self.inner.into_floating_output().into()
    }
    #[inline]
    fn into_pull_up_input(self) -> Input<'a, N, PullUp> {
        self.inner.into_pull_up_input().into()
    }
    #[inline]
    fn into_pull_down_input(self) -> Input<'a, N, PullDown> {
        self.inner.into_pull_down_input().into()
    }
    #[inline]
    fn into_floating_input(self) -> Input<'a, N, Floating> {
        self.inner.into_floating_input().into()
    }
}

#[cfg(any(doc, feature = "glb-v2"))]
impl<'a, const N: usize> IntoPadv2<'a, N> for Disabled<'a, N> {
    #[inline]
    fn into_spi<const I: usize>(self) -> Alternate<'a, N, typestate::Spi<I>> {
        self.inner.into_spi().into()
    }
    #[inline]
    fn into_sdh(self) -> Alternate<'a, N, typestate::Sdh> {
        self.inner.into_sdh().into()
    }
    #[inline]
    fn into_uart(self) -> Alternate<'a, N, typestate::Uart> {
        self.inner.into_uart().into()
    }
    #[inline]
    fn into_mm_uart(self) -> Alternate<'a, N, typestate::MmUart> {
        self.inner.into_mm_uart().into()
    }
    #[inline]
    fn into_pull_up_pwm<const I: usize>(self) -> Alternate<'a, N, typestate::Pwm<I>> {
        self.inner.into_pull_up_pwm().into()
    }
    #[inline]
    fn into_pull_down_pwm<const I: usize>(self) -> Alternate<'a, N, typestate::Pwm<I>> {
        self.inner.into_pull_down_pwm().into()
    }
    #[inline]
    fn into_floating_pwm<const I: usize>(self) -> Alternate<'a, N, typestate::Pwm<I>> {
        self.inner.into_floating_pwm().into()
    }
    #[inline]
    fn into_i2c<const I: usize>(self) -> Alternate<'a, N, typestate::I2c<I>> {
        self.inner.into_i2c().into()
    }
    #[inline]
    fn into_jtag_d0(self) -> Alternate<'a, N, typestate::JtagD0> {
        self.inner.into_jtag_d0().into()
    }
    #[inline]
    fn into_jtag_m0(self) -> Alternate<'a, N, typestate::JtagM0> {
        self.inner.into_jtag_m0().into()
    }
    #[inline]
    fn into_jtag_lp(self) -> Alternate<'a, N, typestate::JtagLp> {
        self.inner.into_jtag_lp().into()
    }
}

impl<'a, const N: usize> From<super::Inner<'a, N, typestate::Disabled>> for Disabled<'a, N> {
    #[inline]
    fn from(inner: super::Inner<'a, N, typestate::Disabled>) -> Self {
        Self { inner }
    }
}
