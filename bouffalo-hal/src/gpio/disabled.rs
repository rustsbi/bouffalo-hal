use super::{
    convert::IntoPad,
    input::Input,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};
use crate::glb::RegisterBlock;
use core::ops::Deref;

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
