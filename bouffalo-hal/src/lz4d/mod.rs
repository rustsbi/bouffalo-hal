//! Hardware LZ4 Decompressor.

mod register;
pub use register::*;

use as_slice::{AsMutSlice, AsSlice};
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

/// Progress on an ongoing decompression procedure.
pub struct Decompress<'a, R, W> {
    lz4d: &'a RegisterBlock,
    resource: Resources<R, W>,
}

impl<'a, R, W> Decompress<'a, R, W> {
    /// Create a new LZ4 decompressor instance.
    #[inline]
    pub fn new(lz4d: impl Instance<'a>, input: Pin<R>, output: Pin<W>) -> Self
    where
        R: Deref + 'a,
        R::Target: AsSlice<Element = u8>,
        W: DerefMut + 'a,
        W::Target: AsMutSlice<Element = u8>,
    {
        let lz4d = lz4d.register_block();
        unsafe {
            lz4d.config.modify(|v| v.disable());
            lz4d.source_start
                .write(SourceStart(input.as_slice().as_ptr() as u32));
            lz4d.destination_start
                .write(DestinationStart(output.as_slice().as_ptr() as u32));
            lz4d.config.modify(|v| v.enable());
        }
        let resource = Resources { input, output };
        Decompress { lz4d, resource }
    }
    /// Checks whether the decompression is still ongoing.
    #[inline]
    pub fn is_ongoing(&self) -> bool {
        !self
            .lz4d
            .interrupt_state
            .read()
            .has_interrupt(Interrupt::Done)
    }
    /// Try to cancel an in process decompression.
    #[inline]
    pub fn cancel(&self) {
        unsafe {
            self.lz4d.config.modify(|v| v.disable());
        }
    }
    /// Waits for the decompression to end.
    #[inline]
    pub fn wait(self) -> Result<(Resources<R, W>, usize), (Resources<R, W>, Error)> {
        loop {
            let state = self.lz4d.interrupt_state.read();
            if state.has_interrupt(Interrupt::Done) {
                let len = self.lz4d.destination_end.read().end()
                    - self.lz4d.destination_start.read().start();
                return Ok((self.resource, len as usize));
            }
            if state.has_interrupt(Interrupt::Error) {
                return Err((self.resource, Error));
            }
            core::hint::spin_loop();
        }
    }
}

/// LZ4 decompressor error.
#[derive(Copy, Clone, Debug)]
pub struct Error;

/// Owned resource pair of decompression.
#[derive(Copy, Clone, Debug)]
pub struct Resources<R, W> {
    /// Decompression input buffer.
    pub input: Pin<R>,
    /// Decompression output buffer.
    pub output: Pin<W>,
}

/// Extend constructor to owned LZ4D register blocks.
pub trait Lz4dExt<'a>: Sized {
    /// Create and start an LZ4D decompression request.
    fn decompress<R, W>(self, input: Pin<R>, output: Pin<W>) -> Decompress<'a, R, W>
    where
        R: Deref + 'a,
        R::Target: AsSlice<Element = u8>,
        W: DerefMut + 'a,
        W::Target: AsMutSlice<Element = u8>;
}

/// Peripheral instance for LZ4D.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}
