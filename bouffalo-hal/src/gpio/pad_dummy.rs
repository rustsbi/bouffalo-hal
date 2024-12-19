#![allow(dead_code)]
use super::typestate::{Floating, Input, Output, PullDown, PullUp};
use crate::glb::Drive;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

pub struct PadDummy<'a, const N: usize, M> {
    _unused: PhantomData<(&'a (), M)>,
}

impl<'a, const N: usize, M> PadDummy<'a, N, Input<M>> {
    #[inline]
    pub fn enable_schmitt(&mut self) {
        unimplemented!()
    }
    #[inline]
    pub fn disable_schmitt(&mut self) {
        unimplemented!()
    }
    #[inline]
    pub fn clear_interrupt(&mut self) {
        unimplemented!()
    }
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        unimplemented!()
    }
    #[inline]
    pub fn mask_interrupt(&mut self) {
        unimplemented!()
    }
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        unimplemented!()
    }
}

impl<'a, const N: usize, M> PadDummy<'a, N, Output<M>> {
    #[inline]
    pub fn drive(&self) -> Drive {
        unimplemented!()
    }
    #[inline]
    pub fn set_drive(&mut self, _: Drive) {
        unimplemented!()
    }
}

impl<'a, const N: usize, M> PadDummy<'a, N, M> {
    #[inline]
    pub fn into_pull_up_output(self) -> PadDummy<'a, N, Output<PullUp>> {
        unimplemented!()
    }
    #[inline]
    pub fn into_pull_down_output(self) -> PadDummy<'a, N, Output<PullDown>> {
        unimplemented!()
    }
    #[inline]
    pub fn into_floating_output(self) -> PadDummy<'a, N, Output<Floating>> {
        unimplemented!()
    }
    #[inline]
    pub fn into_pull_up_input(self) -> PadDummy<'a, N, Input<PullUp>> {
        unimplemented!()
    }
    #[inline]
    pub fn into_pull_down_input(self) -> PadDummy<'a, N, Input<PullDown>> {
        unimplemented!()
    }
    #[inline]
    pub fn into_floating_input(self) -> PadDummy<'a, N, Input<Floating>> {
        unimplemented!()
    }
}

impl<'a, const N: usize, M> ErrorType for PadDummy<'a, N, Input<M>> {
    type Error = core::convert::Infallible;
}

impl<'a, const N: usize, M> ErrorType for PadDummy<'a, N, Output<M>> {
    type Error = core::convert::Infallible;
}

impl<'a, const N: usize, M> InputPin for PadDummy<'a, N, Input<M>> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        unimplemented!()
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        unimplemented!()
    }
}

impl<'a, const N: usize, M> OutputPin for PadDummy<'a, N, Output<M>> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unimplemented!()
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

// Macro internal functions, do not use.
impl<'a, const N: usize> PadDummy<'a, N, super::typestate::Disabled> {
    #[doc(hidden)]
    #[inline]
    pub fn __from_glb(_: &'a crate::glb::RegisterBlock) -> Self {
        Self {
            _unused: PhantomData,
        }
    }
}
