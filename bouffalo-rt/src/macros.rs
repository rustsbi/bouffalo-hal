macro_rules! soc {
    (
        $(
            $(#[$doc:meta])*
            pub struct $Ty:ident => $paddr:expr_2021, $DerefTy:ty;
        )+
    ) => {
        $(
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $Ty {
                _private: (),
            }

            impl $Ty {
                #[inline]
                pub const fn ptr() -> *const $DerefTy {
                    $paddr as *const _
                }
            }

            impl core::ops::Deref for $Ty {
                type Target = $DerefTy;
                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    unsafe { &*($paddr as *const _) }
                }
            }
            impl core::convert::AsRef<$DerefTy> for $Ty {
                #[inline(always)]
                fn as_ref(&self) -> &$DerefTy {
                    unsafe { &*($paddr as *const _) }
                }
            }
        )+
    };
}

macro_rules! dma {
    ($($DMAx: ty: ($x: expr, $WhatChannels: ident, $Periph: ty),)+) => {
$(
    impl<'a> bouffalo_hal::dma::DmaExt for &'a mut $DMAx {
        type Group = $WhatChannels<'a, $Periph>;

        #[inline]
        fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
            $WhatChannels::__new::<$x>(self, glb)
        }
    }

    impl bouffalo_hal::dma::DmaExt for $DMAx {
        type Group = $WhatChannels<'static, $Periph>;

        #[inline]
        fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
            $WhatChannels::__new::<$x>(unsafe { &*Self::ptr() }, glb)
        }
    }
)+
    };
}

macro_rules! uart {
    ($($UARTx: ty: $i: expr,)+) => {
$(
    impl<'a, PADS> bouffalo_hal::uart::UartExt<'a, PADS, $i> for &'a mut $UARTx {
        #[inline]
        fn freerun(
            self,
            config: bouffalo_hal::uart::Config,
            pads: PADS,
            clocks: &Clocks,
        ) -> Result<bouffalo_hal::uart::BlockingSerial<'a, PADS>, bouffalo_hal::uart::ConfigError>
        where
            PADS: bouffalo_hal::uart::Pads<$i>,
        {
            bouffalo_hal::uart::BlockingSerial::__new_freerun(self, config, pads, clocks)
        }
        #[inline]
        fn with_interrupt(
            self,
            config: bouffalo_hal::uart::Config,
            pads: PADS,
            clocks: &Clocks,
            state: &'static bouffalo_hal::uart::SerialState,
        ) -> Result<bouffalo_hal::uart::AsyncSerial<'a, PADS>, bouffalo_hal::uart::ConfigError>
        where
            PADS: bouffalo_hal::uart::Pads<$i>,
        {
            bouffalo_hal::uart::AsyncSerial::__new(self, config, pads, clocks, state)
        }
    }

    impl<PADS> bouffalo_hal::uart::UartExt<'static, PADS, $i> for $UARTx {
        #[inline]
        fn freerun(
            self,
            config: bouffalo_hal::uart::Config,
            pads: PADS,
            clocks: &Clocks,
        ) -> Result<bouffalo_hal::uart::BlockingSerial<'static, PADS>, bouffalo_hal::uart::ConfigError>
        where
            PADS: bouffalo_hal::uart::Pads<$i>,
        {
            bouffalo_hal::uart::BlockingSerial::__new_freerun(unsafe { &*Self::ptr() }, config, pads, clocks)
        }
        #[inline]
        fn with_interrupt(
            self,
            config: bouffalo_hal::uart::Config,
            pads: PADS,
            clocks: &Clocks,
            state: &'static bouffalo_hal::uart::SerialState,
        ) -> Result<bouffalo_hal::uart::AsyncSerial<'static, PADS>, bouffalo_hal::uart::ConfigError>
        where
            PADS: bouffalo_hal::uart::Pads<$i>,
        {
            bouffalo_hal::uart::AsyncSerial::__new(unsafe { &*Self::ptr() }, config, pads, clocks, state)
        }
    }
)+
    };
}

macro_rules! spi {
    ($($SPIx: ty,)+) => {
    $(
impl bouffalo_hal::spi::Instance<'static> for $SPIx {
    #[inline]
    fn register_block(self) -> &'static bouffalo_hal::spi::RegisterBlock {
        unsafe { &*Self::ptr() }
    }
}

impl<'a> bouffalo_hal::spi::Instance<'a> for &'a mut $SPIx {
    #[inline]
    fn register_block(self) -> &'a bouffalo_hal::spi::RegisterBlock {
        &*self
    }
}
    )+
    };
}
