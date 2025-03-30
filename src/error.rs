/// Errors that can occur during the loading process and device tree parsing.
#[derive(Debug)]
pub enum Error<BE>
where
    BE: core::fmt::Debug,
{
    /// An underlying block device error.
    BlockDevice(embedded_sdmmc::Error<BE>),
    /// The file length is too long.
    FileLength(u32),
    /// The file is not a valid DTB.
    InvalidDTB,
    /// The device tree magic number is invalid.
    InvalideMagic(u32),
}

impl<BE> From<embedded_sdmmc::Error<BE>> for Error<BE>
where
    BE: core::fmt::Debug,
{
    #[inline]
    fn from(value: embedded_sdmmc::Error<BE>) -> Self {
        Error::BlockDevice(value)
    }
}
