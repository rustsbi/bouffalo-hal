use super::disabled::Disabled;

/// Available GPIO pads.
pub struct Pads<'a> {
    /// GPIO I/O 0.
    pub io0: Disabled<'a, 0>,
    /// GPIO I/O 1.
    pub io1: Disabled<'a, 1>,
    /// GPIO I/O 2.
    pub io2: Disabled<'a, 2>,
    /// GPIO I/O 3.
    pub io3: Disabled<'a, 3>,
    /// GPIO I/O 4.
    pub io4: Disabled<'a, 4>,
    /// GPIO I/O 5.
    pub io5: Disabled<'a, 5>,
    /// GPIO I/O 6.
    pub io6: Disabled<'a, 6>,
    /// GPIO I/O 7.
    pub io7: Disabled<'a, 7>,
    /// GPIO I/O 8.
    pub io8: Disabled<'a, 8>,
    /// GPIO I/O 9.
    pub io9: Disabled<'a, 9>,
    /// GPIO I/O 10.
    pub io10: Disabled<'a, 10>,
    /// GPIO I/O 11.
    pub io11: Disabled<'a, 11>,
    /// GPIO I/O 12.
    pub io12: Disabled<'a, 12>,
    /// GPIO I/O 13.
    pub io13: Disabled<'a, 13>,
    /// GPIO I/O 14.
    pub io14: Disabled<'a, 14>,
    /// GPIO I/O 15.
    pub io15: Disabled<'a, 15>,
    /// GPIO I/O 16.
    pub io16: Disabled<'a, 16>,
    /// GPIO I/O 17.
    pub io17: Disabled<'a, 17>,
    /// GPIO I/O 18.
    pub io18: Disabled<'a, 18>,
    /// GPIO I/O 19.
    pub io19: Disabled<'a, 19>,
    /// GPIO I/O 20.
    pub io20: Disabled<'a, 20>,
    /// GPIO I/O 21.
    pub io21: Disabled<'a, 21>,
    /// GPIO I/O 22.
    pub io22: Disabled<'a, 22>,
    /// GPIO I/O 23.
    pub io23: Disabled<'a, 23>,
    /// GPIO I/O 24.
    pub io24: Disabled<'a, 24>,
    /// GPIO I/O 25.
    pub io25: Disabled<'a, 25>,
    /// GPIO I/O 26.
    pub io26: Disabled<'a, 26>,
    /// GPIO I/O 27.
    pub io27: Disabled<'a, 27>,
    /// GPIO I/O 28.
    pub io28: Disabled<'a, 28>,
    /// GPIO I/O 29.
    pub io29: Disabled<'a, 29>,
    /// GPIO I/O 30.
    pub io30: Disabled<'a, 30>,
    /// GPIO I/O 31.
    pub io31: Disabled<'a, 31>,
    /// GPIO I/O 32.
    pub io32: Disabled<'a, 32>,
    /// GPIO I/O 33.
    pub io33: Disabled<'a, 33>,
    /// GPIO I/O 34.
    pub io34: Disabled<'a, 34>,
    /// GPIO I/O 35.
    pub io35: Disabled<'a, 35>,
    /// GPIO I/O 36.
    pub io36: Disabled<'a, 36>,
    /// GPIO I/O 37.
    pub io37: Disabled<'a, 37>,
    /// GPIO I/O 38.
    pub io38: Disabled<'a, 38>,
    /// GPIO I/O 39.
    pub io39: Disabled<'a, 39>,
    /// GPIO I/O 40.
    pub io40: Disabled<'a, 40>,
    /// GPIO I/O 41.
    pub io41: Disabled<'a, 41>,
    /// GPIO I/O 42.
    pub io42: Disabled<'a, 42>,
    /// GPIO I/O 43.
    pub io43: Disabled<'a, 43>,
    /// GPIO I/O 44.
    pub io44: Disabled<'a, 44>,
    /// GPIO I/O 45.
    pub io45: Disabled<'a, 45>,
}

// Internal function for macros, do not use.
impl<'a> Pads<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn __pads_from_glb(base: &'a crate::glb::RegisterBlock) -> Self {
        Pads {
            io0: super::Inner::__from_glb(base).into(),
            io1: super::Inner::__from_glb(base).into(),
            io2: super::Inner::__from_glb(base).into(),
            io3: super::Inner::__from_glb(base).into(),
            io4: super::Inner::__from_glb(base).into(),
            io5: super::Inner::__from_glb(base).into(),
            io6: super::Inner::__from_glb(base).into(),
            io7: super::Inner::__from_glb(base).into(),
            io8: super::Inner::__from_glb(base).into(),
            io9: super::Inner::__from_glb(base).into(),
            io10: super::Inner::__from_glb(base).into(),
            io11: super::Inner::__from_glb(base).into(),
            io12: super::Inner::__from_glb(base).into(),
            io13: super::Inner::__from_glb(base).into(),
            io14: super::Inner::__from_glb(base).into(),
            io15: super::Inner::__from_glb(base).into(),
            io16: super::Inner::__from_glb(base).into(),
            io17: super::Inner::__from_glb(base).into(),
            io18: super::Inner::__from_glb(base).into(),
            io19: super::Inner::__from_glb(base).into(),
            io20: super::Inner::__from_glb(base).into(),
            io21: super::Inner::__from_glb(base).into(),
            io22: super::Inner::__from_glb(base).into(),
            io23: super::Inner::__from_glb(base).into(),
            io24: super::Inner::__from_glb(base).into(),
            io25: super::Inner::__from_glb(base).into(),
            io26: super::Inner::__from_glb(base).into(),
            io27: super::Inner::__from_glb(base).into(),
            io28: super::Inner::__from_glb(base).into(),
            io29: super::Inner::__from_glb(base).into(),
            io30: super::Inner::__from_glb(base).into(),
            io31: super::Inner::__from_glb(base).into(),
            io32: super::Inner::__from_glb(base).into(),
            io33: super::Inner::__from_glb(base).into(),
            io34: super::Inner::__from_glb(base).into(),
            io35: super::Inner::__from_glb(base).into(),
            io36: super::Inner::__from_glb(base).into(),
            io37: super::Inner::__from_glb(base).into(),
            io38: super::Inner::__from_glb(base).into(),
            io39: super::Inner::__from_glb(base).into(),
            io40: super::Inner::__from_glb(base).into(),
            io41: super::Inner::__from_glb(base).into(),
            io42: super::Inner::__from_glb(base).into(),
            io43: super::Inner::__from_glb(base).into(),
            io44: super::Inner::__from_glb(base).into(),
            io45: super::Inner::__from_glb(base).into(),
        }
    }
}
