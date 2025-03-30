macro_rules! soc {
    (
        $(
            $(#[$doc:meta])*
            pub struct $Ty:ident => $paddr:expr_2021
            $(, $DerefTy:ty)+ ;
        )+
    ) => {
        $(
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $Ty {
                _private: (),
            }

            $(
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
        )+
    };
}
