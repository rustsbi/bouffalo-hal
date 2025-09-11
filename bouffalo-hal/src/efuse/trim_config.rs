/// Trim device types supported across different chip families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TrimDev {
    // Common device types
    /// 32MHz RC oscillator trim.
    Rc32M,
    /// 32kHz RC oscillator trim.
    Rc32K,
    /// General purpose ADC gain trim.
    Gpadc,
    /// Temperature sensor trim.
    Tsen,
    /// USB 2.0 PHY trim.
    Usb20,

    // Power management types
    /// DCDC trim.
    DcdcTrim,
    /// DCDC output voltage trim.
    DcdcVout,
    /// DCDC disable control.
    DcdcDis,
    /// LDO 1.1V trim.
    Ldo11Trim,
    /// LDO 1.5V trim.
    Ldo15,
    /// LDO 1.8V selection.
    Ldo18Sel,
    /// LDO 1.8V trim.
    Ldo18Trim,
    /// LDO 1.8V bypass control.
    Ldo18Bypass,
    /// LDO 3.3V trim.
    Ldo33Trim,

    // Crystal and reference types
    /// Crystal oscillator trim 0.
    Xtal0,
    /// Crystal oscillator trim 1.
    Xtal1,
    /// Crystal oscillator trim 2.
    Xtal2,
    /// IPTAT reference trim.
    Iptat,
    /// ICX reference trim.
    Icx,

    // BL616 specific offset settings
    /// High power p-offset 0.
    HpPoffset0,
    /// High power p-offset 1.
    HpPoffset1,
    /// High power p-offset 2.
    HpPoffset2,
    /// Low power p-offset 0.
    LpPoffset0,
    /// Low power p-offset 1.
    LpPoffset1,
    /// Low power p-offset 2.
    LpPoffset2,
    /// Buzzer p-offset 0.
    BzPoffset0,
    /// Buzzer p-offset 1.
    BzPoffset1,
    /// Buzzer p-offset 2.
    BzPoffset2,

    // BL616 temperature points
    /// Temperature measure point 0.
    TmpMp0,
    /// Temperature measure point 1.
    TmpMp1,
    /// Temperature measure point 2.
    TmpMp2,

    // Audio ADC
    /// Audio ADC gain trim.
    AuadcGain,
    /// Audio ADC offset trim.
    AuadcOffset,

    // PSRAM configuration
    /// PSRAM trim settings.
    PsramTrim,
}

/// Trim configuration structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrimCfg {
    /// Trim device type.
    pub dev: TrimDev,
    /// Enable bit address.
    pub en_addr: u16,
    /// Parity bit address.
    pub parity_addr: u16,
    /// Value bits starting address.
    pub value_addr: u16,
    /// Value length in bits.
    pub value_len: u8,
}

/// Trim data read from efuse.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrimData {
    /// Enable status.
    pub en: bool,
    /// Trim parity.
    pub parity: bool,
    /// Trim empty.
    pub empty: bool,
    /// Actual trim value.
    pub value: u32,
    /// Length of trim value in bits.
    pub len: u8,
}

// Trim configuration lists for different chips
cfg_if::cfg_if! {
    if #[cfg(feature = "bl808")] {
        /// BL808 trim configuration list.
        const TRIM_CFG_LIST: &[TrimCfg] = &[
            TrimCfg{
                dev: TrimDev::Rc32M,
                en_addr: 0x78 * 8 + 1,
                parity_addr: 0x78 * 8 + 0,
                value_addr: 0x7C * 8 + 4,
                value_len: 8
            },
            TrimCfg{
                dev: TrimDev::Rc32K,
                en_addr: 0xEC * 8 + 19,
                parity_addr: 0xEC * 8 + 18,
                value_addr: 0xEC * 8 + 8,
                value_len: 10
            },
            TrimCfg{
                dev: TrimDev::Gpadc,
                en_addr: 0xF0 * 8 + 27,
                parity_addr: 0xF0 * 8 + 26,
                value_addr: 0xF0 * 8 + 14,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::Tsen,
                en_addr: 0xF0 * 8 + 13,
                parity_addr: 0xF0 * 8 + 12,
                value_addr: 0xF0 * 8 + 0,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::Usb20,
                en_addr: 0xF8 * 8 + 15,
                parity_addr: 0xF8 * 8 + 14,
                value_addr: 0xF8 * 8 + 8,
                value_len: 6
            },
            TrimCfg{
                dev: TrimDev::DcdcTrim,
                en_addr: 0x78 * 8 + 31,
                parity_addr: 0x78 * 8 + 30,
                value_addr: 0x78 * 8 + 26,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo18Sel,
                en_addr: 0x78 * 8 + 25,
                parity_addr: 0x78 * 8 + 24,
                value_addr: 0x78 * 8 + 20,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo33Trim,
                en_addr: 0x78 * 8 + 13,
                parity_addr: 0x78 * 8 + 12,
                value_addr: 0x78 * 8 + 8,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo18Trim,
                en_addr: 0xEC * 8 + 31,
                parity_addr: 0xEC * 8 + 30,
                value_addr: 0xEC * 8 + 26,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo11Trim,
                en_addr: 0xEC * 8 + 25,
                parity_addr: 0xEC * 8 + 24,
                value_addr: 0xEC * 8 + 20,
                value_len: 4
            },
        ];
    } else if #[cfg(feature = "bl616")] {
        /// BL616 trim configuration list.
        const TRIM_CFG_LIST: &[TrimCfg] = &[
            // Power management trims
            TrimCfg{
                dev: TrimDev::Ldo15,
                en_addr: 0x68 * 8 + 31,
                parity_addr: 0x68 * 8 + 30,
                value_addr: 0x68 * 8 + 27,
                value_len: 3
            },
            TrimCfg{
                dev: TrimDev::Iptat,
                en_addr: 0x74 * 8 + 31,
                parity_addr: 0x74 * 8 + 30,
                value_addr: 0x68 * 8 + 22,
                value_len: 5
            },
            TrimCfg{
                dev: TrimDev::Icx,
                en_addr: 0x74 * 8 + 29,
                parity_addr: 0x74 * 8 + 28,
                value_addr: 0x74 * 8 + 22,
                value_len: 6
            },
            TrimCfg{
                dev: TrimDev::DcdcTrim,
                en_addr: 0x78 * 8 + 31,
                parity_addr: 0x78 * 8 + 30,
                value_addr: 0x78 * 8 + 26,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo18Sel,
                en_addr: 0x78 * 8 + 25,
                parity_addr: 0x78 * 8 + 24,
                value_addr: 0x78 * 8 + 20,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo18Trim,
                en_addr: 0x78 * 8 + 19,
                parity_addr: 0x78 * 8 + 18,
                value_addr: 0x78 * 8 + 14,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo33Trim,
                en_addr: 0x78 * 8 + 13,
                parity_addr: 0x78 * 8 + 12,
                value_addr: 0x78 * 8 + 8,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Ldo11Trim,
                en_addr: 0x78 * 8 + 7,
                parity_addr: 0x78 * 8 + 6,
                value_addr: 0x78 * 8 + 2,
                value_len: 4
            },
            TrimCfg{
                dev: TrimDev::Rc32M,
                en_addr: 0x78 * 8 + 1,
                parity_addr: 0x78 * 8 + 0,
                value_addr: 0x7C * 8 + 4,
                value_len: 8
            },
            // Offset configurations
            TrimCfg{
                dev: TrimDev::HpPoffset0,
                en_addr: 0xCC * 8 + 26,
                parity_addr: 0xC0 * 8 + 15,
                value_addr: 0xC0 * 8 + 0,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::HpPoffset1,
                en_addr: 0xCC * 8 + 27,
                parity_addr: 0xC0 * 8 + 31,
                value_addr: 0xC0 * 8 + 16,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::HpPoffset2,
                en_addr: 0xCC * 8 + 28,
                parity_addr: 0xC4 * 8 + 15,
                value_addr: 0xC4 * 8 + 0,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::LpPoffset0,
                en_addr: 0xCC * 8 + 29,
                parity_addr: 0xC4 * 8 + 31,
                value_addr: 0xC4 * 8 + 16,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::LpPoffset1,
                en_addr: 0xCC * 8 + 30,
                parity_addr: 0xC8 * 8 + 15,
                value_addr: 0xC8 * 8 + 0,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::LpPoffset2,
                en_addr: 0xCC * 8 + 31,
                parity_addr: 0xC8 * 8 + 31,
                value_addr: 0xC8 * 8 + 16,
                value_len: 15
            },
            TrimCfg{
                dev: TrimDev::BzPoffset0,
                en_addr: 0xD0 * 8 + 26,
                parity_addr: 0xCC * 8 + 25,
                value_addr: 0xCC * 8 + 0,
                value_len: 25
            },
            TrimCfg{
                dev: TrimDev::BzPoffset1,
                en_addr: 0xD0 * 8 + 27,
                parity_addr: 0xD0 * 8 + 25,
                value_addr: 0xD0 * 8 + 0,
                value_len: 25
            },
            TrimCfg{
                dev: TrimDev::BzPoffset2,
                en_addr: 0xD0 * 8 + 28,
                parity_addr: 0xD4 * 8 + 25,
                value_addr: 0xD4 * 8 + 0,
                value_len: 25
            },
            // Temperature points configuration
            TrimCfg{
                dev: TrimDev::TmpMp0,
                en_addr: 0xD8 * 8 + 9,
                parity_addr: 0xD8 * 8 + 8,
                value_addr: 0xD8 * 8 + 0,
                value_len: 8
            },
            TrimCfg{
                dev: TrimDev::TmpMp1,
                en_addr: 0xD8 * 8 + 19,
                parity_addr: 0xD8 * 8 + 18,
                value_addr: 0xD8 * 8 + 10,
                value_len: 8
            },
            TrimCfg{
                dev: TrimDev::TmpMp2,
                en_addr: 0xD8 * 8 + 29,
                parity_addr: 0xD8 * 8 + 28,
                value_addr: 0xD8 * 8 + 20,
                value_len: 8
            },
            // Audio and analog configurations
            TrimCfg{
                dev: TrimDev::AuadcGain,
                en_addr: 0xDC * 8 + 25,
                parity_addr: 0xDC * 8 + 24,
                value_addr: 0xDC * 8 + 0,
                value_len: 24
            },
            TrimCfg{
                dev: TrimDev::AuadcOffset,
                en_addr: 0xE0 * 8 + 25,
                parity_addr: 0xE0 * 8 + 24,
                value_addr: 0xE0 * 8 + 0,
                value_len: 24
            },
            TrimCfg{
                dev: TrimDev::PsramTrim,
                en_addr: 0xE8 * 8 + 12,
                parity_addr: 0xE8 * 8 + 11,
                value_addr: 0xE8 * 8 + 0,
                value_len: 11
            },
            TrimCfg{
                dev: TrimDev::Rc32K,
                en_addr: 0xEC * 8 + 19,
                parity_addr: 0xEC * 8 + 18,
                value_addr: 0xEC * 8 + 8,
                value_len: 10
            },
            // Crystal configurations
            TrimCfg{
                dev: TrimDev::Xtal0,
                en_addr: 0xEC * 8 + 7,
                parity_addr: 0xEC * 8 + 6,
                value_addr: 0xEC * 8 + 0,
                value_len: 6
            },
            TrimCfg{
                dev: TrimDev::Xtal1,
                en_addr: 0xF0 * 8 + 31,
                parity_addr: 0xF0 * 8 + 30,
                value_addr: 0xF4 * 8 + 26,
                value_len: 6
            },
            TrimCfg{
                dev: TrimDev::Xtal2,
                en_addr: 0xF0 * 8 + 29,
                parity_addr: 0xF0 * 8 + 28,
                value_addr: 0xF4 * 8 + 20,
                value_len: 6
            },
            // ADC and power configurations
            TrimCfg{
                dev: TrimDev::Gpadc,
                en_addr: 0xF0 * 8 + 27,
                parity_addr: 0xF0 * 8 + 26,
                value_addr: 0xF0 * 8 + 14,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::Tsen,
                en_addr: 0xF0 * 8 + 13,
                parity_addr: 0xF0 * 8 + 12,
                value_addr: 0xF0 * 8 + 0,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::DcdcDis,
                en_addr: 0xF4 * 8 + 19,
                parity_addr: 0xF4 * 8 + 18,
                value_addr: 0xF4 * 8 + 17,
                value_len: 1
            },
            TrimCfg{
                dev: TrimDev::DcdcVout,
                en_addr: 0xF4 * 8 + 16,
                parity_addr: 0xF4 * 8 + 15,
                value_addr: 0xF4 * 8 + 10,
                value_len: 5
            },
            TrimCfg{
                dev: TrimDev::Ldo18Bypass,
                en_addr: 0xF4 * 8 + 9,
                parity_addr: 0xF4 * 8 + 8,
                value_addr: 0xF4 * 8 + 4,
                value_len: 1
            },
            TrimCfg{
                dev: TrimDev::Usb20,
                en_addr: 0xF8 * 8 + 15,
                parity_addr: 0xF8 * 8 + 14,
                value_addr: 0xF8 * 8 + 8,
                value_len: 6
            }
        ];
    } else if #[cfg(feature = "bl702")] {
        /// BL702 trim configuration list.
        const TRIM_CFG_LIST: &[TrimCfg] = &[
            TrimCfg{
                dev: TrimDev::Rc32M,
                en_addr: 0x0C * 8 + 19,
                parity_addr: 0x0C * 8 + 18,
                value_addr: 0x0C * 8 + 10,
                value_len: 8
            },
            TrimCfg{
                dev: TrimDev::Rc32K,
                en_addr: 0x0C * 8 + 31,
                parity_addr: 0x0C * 8 + 30,
                value_addr: 0x0C * 8 + 20,
                value_len: 10
            },
            TrimCfg{
                dev: TrimDev::Gpadc,
                en_addr: 0x78 * 8 + 14,
                parity_addr: 0x78 * 8 + 13,
                value_addr: 0x78 * 8 + 1,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::Tsen,
                en_addr: 0x78 * 8 + 0,
                parity_addr: 0x7C * 8 + 12,
                value_addr: 0x7C * 8 + 0,
                value_len: 12
            },
            TrimCfg{
                dev: TrimDev::Xtal0,
                en_addr: 0x0C * 8 + 9,
                parity_addr: 0x0C * 8 + 8,
                value_addr: 0x0C * 8 + 2,
                value_len: 6
            }
        ];
    } else {
        /// Default empty trim configuration list for unsupported chips.
        const TRIM_CFG_LIST: &[TrimCfg] = &[];
    }
}

/// Gets the trim configuration list for the current chip.
pub fn get_trim_cfg_list() -> &'static [TrimCfg] {
    TRIM_CFG_LIST
}

/// Gets the total number of trim configurations.
pub fn get_trim_cfg_count() -> usize {
    TRIM_CFG_LIST.len()
}

/// Find the trim configuration for a specific device type.
pub fn find_trim_cfg(dev: TrimDev) -> Option<&'static TrimCfg> {
    for cfg in TRIM_CFG_LIST {
        if cfg.dev == dev {
            return Some(cfg);
        }
    }
    None
}
