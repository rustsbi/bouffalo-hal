use core::marker::PhantomData;

use crate::gpio::{Alternate, typestate::Spi};

pub struct FlexPad<'a> {
    _base: PhantomData<&'a super::AnyRegisterBlock>,
}

impl<'a> FlexPad<'a> {
    #[inline]
    pub fn from_spi<const N: usize, const F: usize>(pad: Alternate<'a, N, Spi<F>>) -> Self {
        let _ = pad;
        Self { _base: PhantomData }
    }
}
