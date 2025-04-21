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
