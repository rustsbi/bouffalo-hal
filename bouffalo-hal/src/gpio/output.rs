use super::{
    convert::IntoPad,
    input::Input,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::{self, Drive, RegisterBlock};
use core::ops::Deref;
use embedded_hal::digital::{ErrorType, OutputPin};

pub struct Output<GLB, const N: usize, M> {
    #[cfg(feature = "glb-v1")]
    inner: super::pad_v1::Padv1<GLB, N, typestate::Output<M>>,
    #[cfg(feature = "glb-v2")]
    inner: super::pad_v2::Padv2<GLB, N, typestate::Output<M>>,
    #[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
    inner: super::pad_dummy::PadDummy<GLB, N, typestate::Output<M>>,
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> Output<GLB, N, M> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.inner.drive()
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        self.inner.set_drive(val)
    }
}

impl<GLB: Deref<Target = RegisterBlock>, const N: usize, M> IntoPad<GLB, N> for Output<GLB, N, M> {
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

impl<GLB, const N: usize, M> ErrorType for Output<GLB, N, M> {
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M> OutputPin for Output<GLB, N, M> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.inner.set_low()
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.inner.set_high()
    }
}

// This part of implementation using `embedded_hal_027` is designed for backward compatibility of
// ecosystem crates, as some of them depends on embedded-hal v0.2.7 traits.
// We encourage ecosystem developers to use embedded-hal v1.0.0 traits; after that, this part of code
// would be removed in the future.
impl<GLB: Deref<Target = glb::RegisterBlock>, const N: usize, M>
    embedded_hal_027::digital::v2::OutputPin for Output<GLB, N, M>
{
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

#[cfg(feature = "glb-v1")]
impl<GLB, const N: usize, M> From<super::pad_v1::Padv1<GLB, N, typestate::Output<M>>>
    for Output<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_v1::Padv1<GLB, N, typestate::Output<M>>) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "glb-v2")]
impl<GLB, const N: usize, M> From<super::pad_v2::Padv2<GLB, N, typestate::Output<M>>>
    for Output<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_v2::Padv2<GLB, N, typestate::Output<M>>) -> Self {
        Self { inner }
    }
}

#[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
impl<GLB, const N: usize, M> From<super::pad_dummy::PadDummy<GLB, N, typestate::Output<M>>>
    for Output<GLB, N, M>
{
    #[inline]
    fn from(inner: super::pad_dummy::PadDummy<GLB, N, typestate::Output<M>>) -> Self {
        Self { inner }
    }
}
