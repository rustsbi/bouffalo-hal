//! Audio processing peripherals.

use base_address::BaseAddress;
use core::ops;

pub mod auadc;
pub mod audac;

/// Audio Analog-Digital Converter peripheral.
pub struct AUADC<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for AUADC<A> {}

impl<A: BaseAddress> ops::Deref for AUADC<A> {
    type Target = auadc::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Audio Digital-Analog Converter peripheral.
pub struct AUDAC<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for AUDAC<A> {}

impl<A: BaseAddress> ops::Deref for AUDAC<A> {
    type Target = audac::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}
