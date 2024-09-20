//! Universal Serial Bus on BL702 series.
use core::ops;

use volatile_register::{RO, RW, WO};

/// Universal Serial Bus register
#[repr(C)]
pub struct RegisterBlock {
    /// USB configuration
    pub usb_config: RW<UsbConfig>,
    /// USB lpm configuration
    pub usb_lpm_config: RW<UsbLpmConfig>,
    /// USB resume configuration
    pub usb_resume_config: RW<UsbResumeConfig>,
    _reserved0: [u8; 0x0c],
    /// USB frame number
    pub usb_frame_number: RO<UsbFrameNumber>,
    /// USB error
    pub usb_error: RO<UsbError>,
    /// USB interrupt enable
    pub usb_interrupt_enable: RW<UsbInterruptEnable>,
    /// USB interrupt status
    pub usb_interrupt_status: RO<UsbInterruptStatus>,
    /// USB interrupt mask
    pub usb_interrupt_mask: RW<UsbInterruptMask>,
    /// USB interrupt clear
    pub usb_interrupt_clear: WO<UsbInterruptClear>,
    _reserved1: [u8; 0x10],
    /// Endpoint configurations.
    pub endpoint_config: ArrayProxy<RW<EndpointConfig>, 1, 7>,
    _reserved2: [u8; 0xa4],
    /// Endpoint FIFO registers.
    pub endpoint_fifo: [EndpointFifo; 8],
    /// Transceiver interface configuration.
    pub transceiver_config: RW<TransceiverConfig>,
}

/// USB configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbConfig(u32);

/// USB LPM configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbLpmConfig(u32);

/// USB resume configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbResumeConfig(u32);

/// USB frame number register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbFrameNumber(u32);

/// USB error register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbError(u32);

/// USB interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbInterruptEnable(u32);

/// USB interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbInterruptStatus(u32);

/// USB interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbInterruptMask(u32);

/// USB interrupt clear register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UsbInterruptClear(u32);

/// Endpoint configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct EndpointConfig(u32);

/// Endpoint FIFO configurations.
#[repr(C)]
pub struct EndpointFifo {
    /// Endpoint FIFO configuration register.
    pub fifo_config: RW<FifoConfig>,
    /// Endpoint FIFO state register.
    pub fifo_status: RW<FifoStatus>,
    /// Write data into first-in first-out queue.
    pub fifo_write: WO<u32>,
    /// Read data from first-in first-out queue.
    pub fifo_read: RO<u32>,
}

/// Endpoint FIFO configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig(u32);

/// Endpoint FIFO state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoStatus(u32);

/// Transceiver interface configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TransceiverConfig(u32);

/// Array index helper structure.
#[repr(C)]
pub struct ArrayProxy<T, const S: usize, const N: usize> {
    inner: [T; N],
}

impl<T, const S: usize, const N: usize> ops::Index<usize> for ArrayProxy<T, S, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index >= S && index < S + N);
        &self.inner[index - S]
    }
}

#[cfg(test)]
mod tests {
    use super::{EndpointFifo, RegisterBlock};
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, usb_config), 0x00);
        assert_eq!(offset_of!(RegisterBlock, usb_lpm_config), 0x04);
        assert_eq!(offset_of!(RegisterBlock, usb_resume_config), 0x08);
        assert_eq!(offset_of!(RegisterBlock, usb_frame_number), 0x18);
        assert_eq!(offset_of!(RegisterBlock, usb_error), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, usb_interrupt_enable), 0x20);
        assert_eq!(offset_of!(RegisterBlock, usb_interrupt_status), 0x24);
        assert_eq!(offset_of!(RegisterBlock, usb_interrupt_mask), 0x28);
        assert_eq!(offset_of!(RegisterBlock, usb_interrupt_clear), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, endpoint_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, endpoint_fifo), 0x100);
    }

    #[test]
    fn struct_endpoint_fifo_offset() {
        assert_eq!(offset_of!(EndpointFifo, fifo_config), 0x00);
        assert_eq!(offset_of!(EndpointFifo, fifo_status), 0x04);
        assert_eq!(offset_of!(EndpointFifo, fifo_write), 0x08);
        assert_eq!(offset_of!(EndpointFifo, fifo_read), 0x0c);
    }
}
