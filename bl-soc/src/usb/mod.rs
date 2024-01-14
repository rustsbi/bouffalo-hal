//! Universal Serial Bus peripheral.
use base_address::BaseAddress;
use core::ops;

pub mod v1;

/// Inter-IC sound bus peripheral.
pub struct USBv1<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for USBv1<A> {}

impl<A: BaseAddress> ops::Deref for USBv1<A> {
    type Target = v1::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}
