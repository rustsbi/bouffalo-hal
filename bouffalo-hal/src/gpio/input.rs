#[cfg(any(doc, feature = "glb-v2"))]
use super::{alternate::Alternate, convert::IntoPadv2};
use super::{
    convert::IntoPad,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use embedded_hal::digital::{ErrorType, InputPin};

/// GPIO pad in input mode.
pub struct Input<'a, const N: usize, M> {
    inner: super::Inner<'a, N, typestate::Input<M>>,
}

impl<'a, const N: usize, M> Input<'a, N, M> {
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

impl<'a, const N: usize, M> IntoPad<'a, N> for Input<'a, N, M> {
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
impl<'a, const N: usize, M> IntoPadv2<'a, N> for Input<'a, N, M> {
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

impl<'a, const N: usize, M> ErrorType for Input<'a, N, M> {
    type Error = core::convert::Infallible;
}

impl<'a, const N: usize, M> InputPin for Input<'a, N, M> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_high()
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.inner.is_low()
    }
}

impl<'a, const N: usize, M> From<super::Inner<'a, N, typestate::Input<M>>> for Input<'a, N, M> {
    #[inline]
    fn from(inner: super::Inner<'a, N, typestate::Input<M>>) -> Self {
        Self { inner }
    }
}
