use super::{
    alternate::Alternate,
    convert::{IntoPad, IntoPadv2},
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::{self, RegisterBlock};
use core::ops::Deref;
use embedded_hal::digital::{ErrorType, InputPin};

/// GPIO pad in input mode.
pub struct Input<GLB, const N: usize, M> {
    inner: super::Inner<GLB, N, typestate::Input<M>>,
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Input<GLB, N, M> {
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

impl<GLB: Deref<Target = RegisterBlock>, const N: usize, M> IntoPad<GLB, N> for Input<GLB, N, M> {
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
impl<GLB: Deref<Target = RegisterBlock>, const N: usize, M> IntoPadv2<GLB, N> for Input<GLB, N, M> {
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

impl<GLB, const N: usize, M> ErrorType for Input<GLB, N, M> {
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> InputPin for Input<GLB, N, M> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_high()
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_low()
    }
}

impl<GLB, const N: usize, M> From<super::Inner<GLB, N, typestate::Input<M>>> for Input<GLB, N, M> {
    #[inline]
    fn from(inner: super::Inner<GLB, N, typestate::Input<M>>) -> Self {
        Self { inner }
    }
}
