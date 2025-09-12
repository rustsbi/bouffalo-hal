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

// Trim configuration table per chip: indexed by TrimDev as usize. Each
// entry is `Option<TrimCfg>` so unsupported devices are `None`. This makes
// lookups O(1) by indexing with `dev as usize`.
cfg_if::cfg_if! {
    if #[cfg(feature = "bl808")] {
        const TRIM_CFG_LIST: [Option<TrimCfg>; 34] = [
            /*Rc32M*/ Some(TrimCfg{ dev: TrimDev::Rc32M, en_addr: 0x78 * 8 + 1, parity_addr: 0x78 * 8 + 0, value_addr: 0x7C * 8 + 4, value_len: 8 }),
            /*Rc32K*/ Some(TrimCfg{ dev: TrimDev::Rc32K, en_addr: 0xEC * 8 + 19, parity_addr: 0xEC * 8 + 18, value_addr: 0xEC * 8 + 8, value_len: 10 }),
            /*Gpadc*/ Some(TrimCfg{ dev: TrimDev::Gpadc, en_addr: 0xF0 * 8 + 27, parity_addr: 0xF0 * 8 + 26, value_addr: 0xF0 * 8 + 14, value_len: 12 }),
            /*Tsen*/  Some(TrimCfg{ dev: TrimDev::Tsen, en_addr: 0xF0 * 8 + 13, parity_addr: 0xF0 * 8 + 12, value_addr: 0xF0 * 8 + 0, value_len: 12 }),
            /*Usb20*/ Some(TrimCfg{ dev: TrimDev::Usb20, en_addr: 0xF8 * 8 + 15, parity_addr: 0xF8 * 8 + 14, value_addr: 0xF8 * 8 + 8, value_len: 6 }),
            /*DcdcTrim*/ Some(TrimCfg{ dev: TrimDev::DcdcTrim, en_addr: 0x78 * 8 + 31, parity_addr: 0x78 * 8 + 30, value_addr: 0x78 * 8 + 26, value_len: 4 }),
            /*DcdcVout*/ None,
            /*DcdcDis*/ None,
            /*Ldo11Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo11Trim, en_addr: 0xEC * 8 + 25, parity_addr: 0xEC * 8 + 24, value_addr: 0xEC * 8 + 20, value_len: 4 }),
            /*Ldo15*/ None,
            /*Ldo18Sel*/ Some(TrimCfg{ dev: TrimDev::Ldo18Sel, en_addr: 0x78 * 8 + 25, parity_addr: 0x78 * 8 + 24, value_addr: 0x78 * 8 + 20, value_len: 4 }),
            /*Ldo18Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo18Trim, en_addr: 0xEC * 8 + 31, parity_addr: 0xEC * 8 + 30, value_addr: 0xEC * 8 + 26, value_len: 4 }),
            /*Ldo18Bypass*/ None,
            /*Ldo33Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo33Trim, en_addr: 0x78 * 8 + 13, parity_addr: 0x78 * 8 + 12, value_addr: 0x78 * 8 + 8, value_len: 4 }),
            /*Xtal0*/ None,
            /*Xtal1*/ None,
            /*Xtal2*/ None,
            /*Iptat*/ None,
            /*Icx*/ None,
            /*HpPoffset0*/ None,
            /*HpPoffset1*/ None,
            /*HpPoffset2*/ None,
            /*LpPoffset0*/ None,
            /*LpPoffset1*/ None,
            /*LpPoffset2*/ None,
            /*BzPoffset0*/ None,
            /*BzPoffset1*/ None,
            /*BzPoffset2*/ None,
            /*TmpMp0*/ None,
            /*TmpMp1*/ None,
            /*TmpMp2*/ None,
            /*AuadcGain*/ None,
            /*AuadcOffset*/ None,
            /*PsramTrim*/ None,
        ];
    } else if #[cfg(feature = "bl616")] {
        const TRIM_CFG_LIST: [Option<TrimCfg>; 34] = [
            /*Rc32M*/ Some(TrimCfg{ dev: TrimDev::Rc32M, en_addr: 0x78 * 8 + 1, parity_addr: 0x78 * 8 + 0, value_addr: 0x7C * 8 + 4, value_len: 8 }),
            /*Rc32K*/ Some(TrimCfg{ dev: TrimDev::Rc32K, en_addr: 0xEC * 8 + 19, parity_addr: 0xEC * 8 + 18, value_addr: 0xEC * 8 + 8, value_len: 10 }),
            /*Gpadc*/ Some(TrimCfg{ dev: TrimDev::Gpadc, en_addr: 0xF0 * 8 + 27, parity_addr: 0xF0 * 8 + 26, value_addr: 0xF0 * 8 + 14, value_len: 12 }),
            /*Tsen*/  Some(TrimCfg{ dev: TrimDev::Tsen, en_addr: 0xF0 * 8 + 13, parity_addr: 0xF0 * 8 + 12, value_addr: 0xF0 * 8 + 0, value_len: 12 }),
            /*Usb20*/ Some(TrimCfg{ dev: TrimDev::Usb20, en_addr: 0xF8 * 8 + 15, parity_addr: 0xF8 * 8 + 14, value_addr: 0xF8 * 8 + 8, value_len: 6 }),
            /*DcdcTrim*/ Some(TrimCfg{ dev: TrimDev::DcdcTrim, en_addr: 0x78 * 8 + 31, parity_addr: 0x78 * 8 + 30, value_addr: 0x78 * 8 + 26, value_len: 4 }),
            /*DcdcVout*/ Some(TrimCfg{ dev: TrimDev::DcdcVout, en_addr: 0xF4 * 8 + 16, parity_addr: 0xF4 * 8 + 15, value_addr: 0xF4 * 8 + 10, value_len: 5 }),
            /*DcdcDis*/ Some(TrimCfg{ dev: TrimDev::DcdcDis, en_addr: 0xF4 * 8 + 19, parity_addr: 0xF4 * 8 + 18, value_addr: 0xF4 * 8 + 17, value_len: 1 }),
            /*Ldo11Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo11Trim, en_addr: 0x78 * 8 + 7, parity_addr: 0x78 * 8 + 6, value_addr: 0x78 * 8 + 2, value_len: 4 }),
            /*Ldo15*/ Some(TrimCfg{ dev: TrimDev::Ldo15, en_addr: 0x68 * 8 + 31, parity_addr: 0x68 * 8 + 30, value_addr: 0x68 * 8 + 27, value_len: 3 }),
            /*Ldo18Sel*/ Some(TrimCfg{ dev: TrimDev::Ldo18Sel, en_addr: 0x78 * 8 + 25, parity_addr: 0x78 * 8 + 24, value_addr: 0x78 * 8 + 20, value_len: 4 }),
            /*Ldo18Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo18Trim, en_addr: 0x78 * 8 + 19, parity_addr: 0x78 * 8 + 18, value_addr: 0x78 * 8 + 14, value_len: 4 }),
            /*Ldo18Bypass*/ Some(TrimCfg{ dev: TrimDev::Ldo18Bypass, en_addr: 0xF4 * 8 + 9, parity_addr: 0xF4 * 8 + 8, value_addr: 0xF4 * 8 + 4, value_len: 1 }),
            /*Ldo33Trim*/ Some(TrimCfg{ dev: TrimDev::Ldo33Trim, en_addr: 0x78 * 8 + 13, parity_addr: 0x78 * 8 + 12, value_addr: 0x78 * 8 + 8, value_len: 4 }),
            /*Xtal0*/ Some(TrimCfg{ dev: TrimDev::Xtal0, en_addr: 0xEC * 8 + 7, parity_addr: 0xEC * 8 + 6, value_addr: 0xEC * 8 + 0, value_len: 6 }),
            /*Xtal1*/ Some(TrimCfg{ dev: TrimDev::Xtal1, en_addr: 0xF0 * 8 + 31, parity_addr: 0xF0 * 8 + 30, value_addr: 0xF4 * 8 + 26, value_len: 6 }),
            /*Xtal2*/ Some(TrimCfg{ dev: TrimDev::Xtal2, en_addr: 0xF0 * 8 + 29, parity_addr: 0xF0 * 8 + 28, value_addr: 0xF4 * 8 + 20, value_len: 6 }),
            /*Iptat*/ Some(TrimCfg{ dev: TrimDev::Iptat, en_addr: 0x74 * 8 + 31, parity_addr: 0x74 * 8 + 30, value_addr: 0x68 * 8 + 22, value_len: 5 }),
            /*Icx*/ Some(TrimCfg{ dev: TrimDev::Icx, en_addr: 0x74 * 8 + 29, parity_addr: 0x74 * 8 + 28, value_addr: 0x74 * 8 + 22, value_len: 6 }),
            /*HpPoffset0*/ Some(TrimCfg{ dev: TrimDev::HpPoffset0, en_addr: 0xCC * 8 + 26, parity_addr: 0xC0 * 8 + 15, value_addr: 0xC0 * 8 + 0, value_len: 15 }),
            /*HpPoffset1*/ Some(TrimCfg{ dev: TrimDev::HpPoffset1, en_addr: 0xCC * 8 + 27, parity_addr: 0xC0 * 8 + 31, value_addr: 0xC0 * 8 + 16, value_len: 15 }),
            /*HpPoffset2*/ Some(TrimCfg{ dev: TrimDev::HpPoffset2, en_addr: 0xCC * 8 + 28, parity_addr: 0xC4 * 8 + 15, value_addr: 0xC4 * 8 + 0, value_len: 15 }),
            /*LpPoffset0*/ Some(TrimCfg{ dev: TrimDev::LpPoffset0, en_addr: 0xCC * 8 + 29, parity_addr: 0xC4 * 8 + 31, value_addr: 0xC4 * 8 + 16, value_len: 15 }),
            /*LpPoffset1*/ Some(TrimCfg{ dev: TrimDev::LpPoffset1, en_addr: 0xCC * 8 + 30, parity_addr: 0xC8 * 8 + 15, value_addr: 0xC8 * 8 + 0, value_len: 15 }),
            /*LpPoffset2*/ Some(TrimCfg{ dev: TrimDev::LpPoffset2, en_addr: 0xCC * 8 + 31, parity_addr: 0xC8 * 8 + 31, value_addr: 0xC8 * 8 + 16, value_len: 15 }),
            /*BzPoffset0*/ Some(TrimCfg{ dev: TrimDev::BzPoffset0, en_addr: 0xD0 * 8 + 26, parity_addr: 0xCC * 8 + 25, value_addr: 0xCC * 8 + 0, value_len: 25 }),
            /*BzPoffset1*/ Some(TrimCfg{ dev: TrimDev::BzPoffset1, en_addr: 0xD0 * 8 + 27, parity_addr: 0xD0 * 8 + 25, value_addr: 0xD0 * 8 + 0, value_len: 25 }),
            /*BzPoffset2*/ Some(TrimCfg{ dev: TrimDev::BzPoffset2, en_addr: 0xD0 * 8 + 28, parity_addr: 0xD4 * 8 + 25, value_addr: 0xD4 * 8 + 0, value_len: 25 }),
            /*TmpMp0*/ Some(TrimCfg{ dev: TrimDev::TmpMp0, en_addr: 0xD8 * 8 + 9, parity_addr: 0xD8 * 8 + 8, value_addr: 0xD8 * 8 + 0, value_len: 8 }),
            /*TmpMp1*/ Some(TrimCfg{ dev: TrimDev::TmpMp1, en_addr: 0xD8 * 8 + 19, parity_addr: 0xD8 * 8 + 18, value_addr: 0xD8 * 8 + 10, value_len: 8 }),
            /*TmpMp2*/ Some(TrimCfg{ dev: TrimDev::TmpMp2, en_addr: 0xD8 * 8 + 29, parity_addr: 0xD8 * 8 + 28, value_addr: 0xD8 * 8 + 20, value_len: 8 }),
            /*AuadcGain*/ Some(TrimCfg{ dev: TrimDev::AuadcGain, en_addr: 0xDC * 8 + 25, parity_addr: 0xDC * 8 + 24, value_addr: 0xDC * 8 + 0, value_len: 24 }),
            /*AuadcOffset*/ Some(TrimCfg{ dev: TrimDev::AuadcOffset, en_addr: 0xE0 * 8 + 25, parity_addr: 0xE0 * 8 + 24, value_addr: 0xE0 * 8 + 0, value_len: 24 }),
            /*PsramTrim*/ Some(TrimCfg{ dev: TrimDev::PsramTrim, en_addr: 0xE8 * 8 + 12, parity_addr: 0xE8 * 8 + 11, value_addr: 0xE8 * 8 + 0, value_len: 11 }),
        ];
    } else if #[cfg(feature = "bl702")] {
        const TRIM_CFG_LIST: [Option<TrimCfg>; 34] = [
            /*Rc32M*/ Some(TrimCfg{ dev: TrimDev::Rc32M, en_addr: 0x0C * 8 + 19, parity_addr: 0x0C * 8 + 18, value_addr: 0x0C * 8 + 10, value_len: 8 }),
            /*Rc32K*/ Some(TrimCfg{ dev: TrimDev::Rc32K, en_addr: 0x0C * 8 + 31, parity_addr: 0x0C * 8 + 30, value_addr: 0x0C * 8 + 20, value_len: 10 }),
            /*Gpadc*/ Some(TrimCfg{ dev: TrimDev::Gpadc, en_addr: 0x78 * 8 + 14, parity_addr: 0x78 * 8 + 13, value_addr: 0x78 * 8 + 1, value_len: 12 }),
            /*Tsen*/  Some(TrimCfg{ dev: TrimDev::Tsen, en_addr: 0x78 * 8 + 0, parity_addr: 0x7C * 8 + 12, value_addr: 0x7C * 8 + 0, value_len: 12 }),
            /*Usb20*/ None,
            /*DcdcTrim*/ None,
            /*DcdcVout*/ None,
            /*DcdcDis*/ None,
            /*Ldo11Trim*/ None,
            /*Ldo15*/ None,
            /*Ldo18Sel*/ None,
            /*Ldo18Trim*/ None,
            /*Ldo18Bypass*/ None,
            /*Ldo33Trim*/ None,
            /*Xtal0*/ Some(TrimCfg{ dev: TrimDev::Xtal0, en_addr: 0x0C * 8 + 9, parity_addr: 0x0C * 8 + 8, value_addr: 0x0C * 8 + 2, value_len: 6 }),
            /*Xtal1*/ None,
            /*Xtal2*/ None,
            /*Iptat*/ None,
            /*Icx*/ None,
            /*HpPoffset0*/ None,
            /*HpPoffset1*/ None,
            /*HpPoffset2*/ None,
            /*LpPoffset0*/ None,
            /*LpPoffset1*/ None,
            /*LpPoffset2*/ None,
            /*BzPoffset0*/ None,
            /*BzPoffset1*/ None,
            /*BzPoffset2*/ None,
            /*TmpMp0*/ None,
            /*TmpMp1*/ None,
            /*TmpMp2*/ None,
            /*AuadcGain*/ None,
            /*AuadcOffset*/ None,
            /*PsramTrim*/ None,
        ];
    } else {
        const TRIM_CFG_LIST: [Option<TrimCfg>; 34] = [None; 34];
    }
}

/// Gets the total number of trim configurations.
pub fn get_trim_cfg_count() -> usize {
    // Number of enum variants (size of per-device table).
    TRIM_CFG_LIST.len()
}

/// O(1) lookup: get trim configuration by device using direct indexing.
pub fn get_trim_cfg_by_dev(dev: TrimDev) -> Option<&'static TrimCfg> {
    let idx = dev as usize;
    if idx >= TRIM_CFG_LIST.len() {
        return None;
    }
    match &TRIM_CFG_LIST[idx] {
        Some(cfg) => Some(cfg),
        None => None,
    }
}
