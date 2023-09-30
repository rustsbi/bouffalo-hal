//! Global configuration peripheral.

use base_address::BaseAddress;
use core::ops;

pub mod mm;
pub mod v1;
pub mod v2;

/// Global configurations on BL602 and BL702 series.
pub struct GLBv1<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for GLBv1<A> {}

impl<A: BaseAddress> ops::Deref for GLBv1<A> {
    type Target = v1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Global configurations on BL808 and BL616 series.
pub struct GLBv2<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for GLBv2<A> {}

impl<A: BaseAddress> ops::Deref for GLBv2<A> {
    type Target = v2::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Multi-media subsystem global peripheral.
pub struct MMGLB<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for MMGLB<A> {}

impl<A: BaseAddress> ops::Deref for MMGLB<A> {
    type Target = v2::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}
