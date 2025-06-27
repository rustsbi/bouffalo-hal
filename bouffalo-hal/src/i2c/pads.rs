use crate::gpio::FlexPad;

/// Pad that can be configured into I2C SCL alternate function.
pub trait IntoI2cScl<'a, const I: usize> {
    /// Configure this pad into I2C SCL signal.
    fn into_i2c_scl(self) -> FlexPad<'a>;
}

/// Pad that can be configured into I2C SDA alternate function.
pub trait IntoI2cSda<'a, const I: usize> {
    /// Configure this pad into I2C SDA signal.
    fn into_i2c_sda(self) -> FlexPad<'a>;
}

/// Pads that can be converted into valid I2C pads.
pub trait IntoPads<'a, const I: usize> {
    /// Convert this set of pad into I2C alternate function.
    fn into_i2c_pads(self) -> (FlexPad<'a>, FlexPad<'a>);
}

impl<'a, A, B, const I: usize> IntoPads<'a, I> for (A, B)
where
    A: IntoI2cScl<'a, I>,
    B: IntoI2cSda<'a, I>,
{
    #[inline]
    fn into_i2c_pads(self) -> (FlexPad<'a>, FlexPad<'a>) {
        let a = self.0.into_i2c_scl();
        let b = self.1.into_i2c_sda();
        (a, b)
    }
}
