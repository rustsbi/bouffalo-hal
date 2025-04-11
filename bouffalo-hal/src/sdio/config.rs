use super::register::{BusWidthMode, SpeedMode, TransferWidth};

/// SDH hardware initial config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    pub bus_width_mode: BusWidthMode,
    pub transfer_width: TransferWidth,
    pub speed_mode: SpeedMode,
    // TODO: implment more configurations if necessary.
}

impl Config {
    /// Default SDH config.
    #[inline]
    pub const fn default() -> Self {
        Self {
            bus_width_mode: BusWidthMode::SelectByDataTransferWidth,
            transfer_width: TransferWidth::OneBitMode,
            speed_mode: SpeedMode::HighSpeed,
        }
    }
    /// Set bus width mode.
    #[inline]
    pub const fn bus_width_mode(mut self, bus_width_mode: BusWidthMode) -> Self {
        self.bus_width_mode = bus_width_mode;
        self
    }
    /// Set transfer width.
    #[inline]
    pub const fn transfer_width(mut self, transfer_width: TransferWidth) -> Self {
        self.transfer_width = transfer_width;
        self
    }
    /// Set speed mode.
    #[inline]
    pub const fn speed_mode(mut self, speed_mode: SpeedMode) -> Self {
        self.speed_mode = speed_mode;
        self
    }
}
