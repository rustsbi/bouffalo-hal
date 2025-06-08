pub trait SclPin<const I: usize> {}

pub trait SdaPin<const I: usize> {}

#[rustfmt::skip]
mod i2c_impls {
    use crate::gpio::{self, Alternate};
    use super::{SclPin, SdaPin};

    // 0, 2, 4, ..., 2n: SCL
    // 1, 3, 5, ..., 2n+1: SDA
    // TODO: support other pads if needed
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 0, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 1, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 2, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 3, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 4, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 5, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 6, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 7, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 8, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 9, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 10, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 11, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 12, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 13, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 14, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 15, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 16, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 17, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 18, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 19, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 20, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 21, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 22, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 23, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 24, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 25, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 26, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 27, gpio::I2c<I>> {}
    impl<'a, const I: usize> SclPin<I> for Alternate<'a, 28, gpio::I2c<I>> {}
    impl<'a, const I: usize> SdaPin<I> for Alternate<'a, 29, gpio::I2c<I>> {}
}
