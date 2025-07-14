//! SoC peripheral support for bouffalolab chips.
//!
//! This package is built under the concept of componentized drivers. It is designed to be
//! used in kernels, firmwares and embedded development with both dynamic and static base
//! address support.
//!
//! Most of `bouffalo-hal` structures have `embedded-hal` traits implemented. Users may combine
//! this package with `embedded-hal` ecosystem drivers to provide abundant amount of features.
#![no_std]

pub mod clocks;

pub mod audio;
pub mod dbi;
pub mod dma;
pub mod efuse;
pub mod emac;
pub mod glb;
pub mod gpio;
pub mod gpip;
pub mod hbn;
pub mod i2c;
pub mod i2s;
pub mod ir;
pub mod lz4d;
pub mod psram;
pub mod pwm;
pub mod sdio;
pub mod sec;
pub mod spi;
pub mod timer;
pub mod uart;
pub mod usb;

#[doc(hidden)]
pub mod prelude {
    pub use crate::dma::DmaExt as _;
    pub use crate::gpio::IntoPad as _;
    #[cfg(feature = "glb-v2")]
    pub use crate::gpio::IntoPadv2 as _;
    pub use crate::lz4d::Lz4dExt as _;
    pub use crate::uart::{IntoUartSignal as _, UartExt as _};
    pub use embedded_hal::digital::{InputPin as _, OutputPin as _, PinState};
    pub use embedded_hal::i2c::I2c as _;
    pub use embedded_hal::pwm::SetDutyCycle as _;
    pub use embedded_io::{Read as _, Write as _};
    pub use embedded_io_async::{Read as _, Write as _};
}

/// Wrapper type for manipulations of a field in a register.
///
/// * LEN: the length of the field in bits.
/// * OFFSET: the bit number counted from the bit 0 to the first bit of the field.
/// * T: the inner type representing the register with the same size as T.
///
/// Note: size of T should be smaller than size of usize, largest possible type is u64.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct BitField<const LEN: usize, const SHIFT: usize, T: Sized + Copy> {
    v: T,
}

// TODO: replace this with trait From when 'const_trait' is stable
macro_rules! impl_from_for_register_field {
    ($($T: ty,)+) => {
$(
impl<const LEN: usize, const SHIFT: usize> BitField<LEN, SHIFT, $T> {
    #[allow(unused)]
    #[inline(always)]
    pub const fn from(value: $T) -> Self
    // TODO: put LEN and SHIFT check in where clause when 'generic_const_exprs' is stable
    // where
    //     [(); LEN - 1]:,
    //     [(); core::mem::size_of::<$T>() * 8 - SHIFT - LEN]:,
    {
        // Can drop extra bits silently but it indicates potential coding problems
        debug_assert!(LEN >= 1 && (LEN + SHIFT) <= core::mem::size_of::<$T>() * 8);
        Self { v: value }
    }
}
)+
    }
}

// TODO: replace this with trait to avoid impl. duplication by macro when 'const_trait' is stable
macro_rules! impl_register_field {
    ($($T: ty,)+) => {
$(
impl<const LEN: usize, const SHIFT: usize> BitField<LEN, SHIFT, $T> {
    /// Set a value to the field in a register without boundary check.
    #[allow(unused)]
    #[inline(always)]
    pub const fn set(self, val: usize) -> $T {
        let mask = self.get_mask();
        let data = (self.v as usize) & !mask | ((val << SHIFT) & mask);
        data as $T
    }
    /// Get the value of the field in a register.
    #[inline(always)]
    pub const fn get(self) -> usize {
        let mask = self.get_mask();
        ((self.v as usize) & mask) >> SHIFT
    }
    /// Set a value to the field in a register with boundary check.
    #[allow(unused)]
    #[inline(always)]
    pub const fn checked_set(self, val: usize) -> Option<$T> {
        let mask = self.get_mask();
        let data = (self.v as usize) & !mask | ((val << SHIFT) & mask);
        if val > (mask >> SHIFT) {
            None
        } else {
            Some(data as $T)
        }
    }
    /// Enable the function controlled by the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn enable(self) -> $T {
        debug_assert!(LEN == 1);
        let data = (self.v as usize) | (1 << SHIFT);
        data as $T
    }
    /// Disable the function controlled by the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn disable(self) -> $T {
        debug_assert!(LEN == 1);
        let data = (self.v as usize) & !(1 << SHIFT);
        data as $T
    }
    /// Check if the function controlled by the field in a register is enabled
    #[allow(unused)]
    #[inline(always)]
    pub const fn is_enabled(self) -> bool {
        debug_assert!(LEN == 1);
        self.get() != 0
    }
    /// Toggle the value of the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn toggle(self) -> $T {
        debug_assert!(LEN == 1);
        let data = (self.v as usize) ^ (1 << SHIFT);
        data as $T
    }
    /// Clear the value of the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn clear(self) -> $T { self.set(0) }
    /// Get the mask bits of the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn get_mask(self) -> usize {
        ((((1 as $T) << LEN) - 1) << SHIFT) as usize
    }
    /// Get the shift of the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn get_shift(self) -> usize { SHIFT }
    /// Get the length of the field in a register
    #[allow(unused)]
    #[inline(always)]
    pub const fn get_len(self) -> usize { LEN }
}
)+
    };
}

impl_from_for_register_field! { u8, u16, u32, u64, usize, }
impl_register_field! { u8, u16, u32, u64, usize, }

#[cfg(test)]
mod tests {
    use super::BitField;

    #[test]
    fn struct_register_field() {
        type Field1 = BitField<2, 6, u8>;
        type Field2 = BitField<2, 7, u16>;
        type Field3 = BitField<1, 0, u32>;
        type Field4 = BitField<3, 6, u32>;
        type Field5 = BitField<2, 62, u64>;

        // tests for register with 8 bits width
        let mut field = Field1::from(0);

        assert_eq!(field.get_len(), 2);
        assert_eq!(field.get_shift(), 6);
        assert_eq!(field.get_mask(), 0xC0);

        field = Field1::from(field.set(2));
        assert_eq!(field.v, 0x80);
        assert_eq!(field.get(), 2);

        field = Field1::from(field.set(5));
        assert_eq!(field.get(), 1);
        assert_eq!(field.v, 0x40);

        let field = match field.checked_set(5) {
            Some(val) => Field1::from(val),
            None => Field1::from(0xF0),
        };
        assert_eq!(field.get(), 3);
        assert_eq!(field.v, 0xF0);

        // tests for register with 16 bits width
        let mut field = Field2::from(0x0040);

        field = Field2::from(field.set(1));
        assert_eq!(field.get(), 1);
        assert_eq!(field.v, 0x00C0);

        field = Field2::from(field.set(7));
        assert_eq!(field.get(), 3);
        assert_eq!(field.v, 0x01C0);

        // tests for register with 32 bits width
        let mut field = Field3::from(0);

        field = Field3::from(field.enable());
        assert_eq!(field.get(), 1);
        assert!(field.is_enabled());
        assert_eq!(field.v, 1);

        field = Field3::from(field.disable());
        assert_eq!(field.get(), 0);
        assert!(!field.is_enabled());
        assert_eq!(field.v, 0);

        field = Field3::from(field.set(2));
        assert_eq!(field.get(), 0);
        assert_eq!(field.v, 0);

        let mut field = Field4::from(0);

        field = Field4::from(field.set(2));
        assert_eq!(field.get(), 2);
        assert_eq!(field.v, 0x0000_0080);

        field = Field4::from(field.set(9));
        assert_eq!(field.get(), 1);
        assert_eq!(field.v, 0x0000_0040);

        field = Field4::from(field.clear());
        assert_eq!(field.get(), 0);
        assert_eq!(field.v, 0x0000_0000);

        // tests for register with 64 bits width
        let mut field = Field5::from(0x2000_0000_0000_0000u64);

        field = Field5::from(field.set(2));
        assert_eq!(field.get(), 2);
        assert_eq!(field.v, 0xA000_0000_0000_0000);

        field = Field5::from(field.set(9));
        assert_eq!(field.get(), 1);
        assert_eq!(field.v, 0x6000_0000_0000_0000);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn struct_register_field_panic_oversize() {
        type Field1 = BitField<3, 6, u8>;

        Field1::from(0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn struct_register_field_panic_enable_multibits() {
        type Field1 = BitField<2, 6, u8>;

        let field = Field1::from(0);
        field.enable();
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn struct_register_field_panic_zero_len() {
        type Field1 = BitField<0, 1, u32>;

        Field1::from(0);
    }
}
