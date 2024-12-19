use super::{
    convert::IntoPad,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::{self, RegisterBlock};
use core::ops::Deref;
use embedded_hal::digital::{ErrorType, InputPin};

pub struct Input<GLB, const N: usize, M> {
    #[cfg(feature = "glb-v1")]
    inner: super::pad_v1::Padv1<GLB, N, typestate::Input<M>>,
    #[cfg(feature = "glb-v2")]
    inner: super::pad_v2::Padv2<GLB, N, typestate::Input<M>>,
    #[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
    inner: super::pad_dummy::PadDummy<GLB, N, typestate::Input<M>>,
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

#[cfg(feature = "glb-v1")]
impl<GLB, const N: usize, M> From<super::pad_v1::Padv1<GLB, N, typestate::Input<M>>>
    for Input<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_v1::Padv1<GLB, N, typestate::Input<M>>) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "glb-v2")]
impl<GLB, const N: usize, M> From<super::pad_v2::Padv2<GLB, N, typestate::Input<M>>>
    for Input<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_v2::Padv2<GLB, N, typestate::Input<M>>) -> Self {
        Self { inner }
    }
}

#[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
impl<GLB, const N: usize, M> From<super::pad_dummy::PadDummy<GLB, N, typestate::Input<M>>>
    for Input<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_dummy::PadDummy<GLB, N, typestate::Input<M>>) -> Self {
        Self { inner }
    }
}
