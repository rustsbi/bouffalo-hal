#![no_std]

use base_address::BaseAddress;
use core::ops;

pub mod glb;
pub mod gpio;
pub mod jtag;
pub mod uart;
pub mod hbn;

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

/// Universal Asynchronous Receiver/Transmitter.
pub struct UART<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for UART<A> {}

impl<A: BaseAddress> ops::Deref for UART<A> {
    type Target = uart::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}

/// Hibernation (deep-sleep) control.
pub struct HBN<A: BaseAddress> {
    base: A,
}

unsafe impl<A: BaseAddress> Send for HBN<A> {}

impl<A: BaseAddress> ops::Deref for HBN<A> {
    type Target = hbn::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base.ptr() as *const _) }
    }
}