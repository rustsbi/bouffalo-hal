//! Global configuration peripheral.

pub mod mm;
pub mod v1;
pub mod v2;

/// Pin pull direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    /// No internal pulls.
    None = 0,
    /// Internally pulled up.
    Up = 1,
    /// Internally pulled down.
    Down = 2,
}

/// Pin drive strength.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drive {
    /// Drive strength 0.
    Drive0 = 0,
    /// Drive strength 1.
    Drive1 = 1,
    /// Drive strength 2.
    Drive2 = 2,
    /// Drive strength 3.
    Drive3 = 3,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "glb-v1")] {
        pub use v1::RegisterBlock;
    } else if #[cfg(feature = "glb-v2")] {
        pub use v2::RegisterBlock;
    } else {
        /// Global configuration registers.
        pub struct RegisterBlock {}
    }
}
