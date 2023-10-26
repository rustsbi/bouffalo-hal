//! Universal Serial Bus on BL702 series.
use volatile_register::RW;

/// Universal Serial Bus register
#[repr(C)]
pub struct RegisterBlock {
    /// USB configuration
    pub usb_config: RW<UsbConfig>,
    /// USB lpm configuration
    pub usb_lpm_config: RW<UsbLpmConfig>,
    /// USB resume configuration
    pub usb_resume_config: RW<UsbResumeConfig>,
    // /// USB frame number
    //
    // USB error
    //
    // USB interrupt enable
    //
    // USB interrupt status
    //
    // USB interrupt mask
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

// /// USB frame number register.
