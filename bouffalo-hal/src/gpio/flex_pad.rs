use crate::gpio::{
    Alternate,
    typestate::{I2c, Spi},
};
use core::marker::PhantomData;

pub struct FlexPad<'a> {
    _base: PhantomData<&'a super::AnyRegisterBlock>,
}

impl<'a> FlexPad<'a> {
    #[inline]
    pub fn from_spi<const N: usize, const F: usize>(pad: Alternate<'a, N, Spi<F>>) -> Self {
        let _ = pad;
        Self { _base: PhantomData }
    }
    #[inline]
    pub fn from_i2c<const N: usize, const F: usize>(pad: Alternate<'a, N, I2c<F>>) -> Self {
        let _ = pad;
        Self { _base: PhantomData }
    }
}
