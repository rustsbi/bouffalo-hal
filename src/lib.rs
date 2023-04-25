#![no_std]

use base_address::BaseAddress;
use core::ops;

pub mod glb;
pub mod gpio;

/// Global register.
pub struct GLB<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for GLB<A> {}

impl<A: BaseAddress> ops::Deref for GLB<A> {
    type Target = glb::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}
