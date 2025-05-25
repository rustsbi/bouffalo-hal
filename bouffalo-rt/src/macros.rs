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
    impl bouffalo_hal::dma::Instance<'static> for $DMAx {
        #[inline]
        fn register_block(self) -> &'static bouffalo_hal::dma::RegisterBlock {
            unsafe { &*Self::ptr() }
        }
    }

    impl<'a> bouffalo_hal::dma::Instance<'a> for &'a mut $DMAx {
        #[inline]
        fn register_block(self) -> &'a bouffalo_hal::dma::RegisterBlock {
            &*self
        }
    }

    impl<'a> bouffalo_hal::dma::DmaExt for &'a mut $DMAx {
        type Group = $WhatChannels<'a, $Periph>;

        #[inline]
        fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
            $WhatChannels::new::<$x>(self, glb)
        }
    }

    impl bouffalo_hal::dma::DmaExt for $DMAx {
        type Group = $WhatChannels<'static, $Periph>;

        #[inline]
        fn split(self, glb: &bouffalo_hal::glb::v2::RegisterBlock) -> Self::Group {
            $WhatChannels::new::<$x>(self, glb)
        }
    }
)+
    };
}

macro_rules! uart {
    ($($UARTx: ty: $i: expr,)+) => {
$(
    impl bouffalo_hal::uart::Instance<'static> for $UARTx {
        #[inline]
        fn register_block(self) -> &'static bouffalo_hal::uart::RegisterBlock {
            unsafe { &*Self::ptr() }
        }
    }

    impl<'a> bouffalo_hal::uart::Instance<'a> for &'a mut $UARTx {
        #[inline]
        fn register_block(self) -> &'a bouffalo_hal::uart::RegisterBlock {
            &*self
        }
    }

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
            bouffalo_hal::uart::BlockingSerial::new_freerun(self, config, pads, clocks)
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
            bouffalo_hal::uart::AsyncSerial::new(self, config, pads, clocks, state)
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
            bouffalo_hal::uart::BlockingSerial::new_freerun(self, config, pads, clocks)
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
            bouffalo_hal::uart::AsyncSerial::new(self, config, pads, clocks, state)
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

macro_rules! pwm {
    ($($PWMx: ty,)+) => {
    $(
impl bouffalo_hal::pwm::Instance<'static> for $PWMx {
    #[inline]
    fn register_block(self) -> &'static bouffalo_hal::pwm::RegisterBlock {
        unsafe { &*Self::ptr() }
    }
}

impl<'a> bouffalo_hal::pwm::Instance<'a> for &'a mut $PWMx {
    #[inline]
    fn register_block(self) -> &'a bouffalo_hal::pwm::RegisterBlock {
        &*self
    }
}
    )+
    };
}

macro_rules! impl_pad_v2 {
    ($Pad: ident: $GLBv2: ident) => {
        impl<'a, const N: usize> bouffalo_hal::gpio::Instance<'a> for &'a mut $Pad<N> {
            #[inline]
            fn register_block(self) -> &'a bouffalo_hal::gpio::AnyRegisterBlock {
                <&bouffalo_hal::gpio::AnyRegisterBlock as From<
                    &bouffalo_hal::glb::v2::RegisterBlock,
                >>::from(&$GLBv2 { _private: () })
            }
            #[inline]
            fn version(&self) -> bouffalo_hal::glb::Version {
                bouffalo_hal::glb::Version::V2
            }
        }

        impl<'a, const N: usize> bouffalo_hal::gpio::Numbered<'a, N> for &'a mut $Pad<N> {}

        impl<const N: usize> bouffalo_hal::gpio::Instance<'static> for $Pad<N> {
            #[inline]
            fn register_block(self) -> &'static bouffalo_hal::gpio::AnyRegisterBlock {
                <&bouffalo_hal::gpio::AnyRegisterBlock as From<
                    &bouffalo_hal::glb::v2::RegisterBlock,
                >>::from(&$GLBv2 { _private: () })
            }
            #[inline]
            fn version(&self) -> bouffalo_hal::glb::Version {
                bouffalo_hal::glb::Version::V2
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::Numbered<'static, N> for $Pad<N> {}

        impl<'a, const N: usize> bouffalo_hal::gpio::pad_v2::Instance<'a> for &'a mut $Pad<N> {
            #[inline]
            fn register_block(self) -> &'a bouffalo_hal::glb::v2::RegisterBlock {
                &$GLBv2 { _private: () }
            }
        }

        impl<'a, const N: usize> bouffalo_hal::gpio::pad_v2::Numbered<'a, N> for &'a mut $Pad<N> {}

        impl<const N: usize> bouffalo_hal::gpio::pad_v2::Instance<'static> for $Pad<N> {
            #[inline]
            fn register_block(self) -> &'static bouffalo_hal::glb::v2::RegisterBlock {
                &$GLBv2 { _private: () }
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::pad_v2::Numbered<'static, N> for $Pad<N> {}

        impl<'a, const N: usize> bouffalo_hal::gpio::IntoPad<'a> for &'a mut $Pad<N> {
            #[inline]
            fn into_pull_up_output(self) -> bouffalo_hal::gpio::Output<'a> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_output(self) -> bouffalo_hal::gpio::Output<'a> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_output(self) -> bouffalo_hal::gpio::Output<'a> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::None)
            }
            #[inline]
            fn into_pull_up_input(self) -> bouffalo_hal::gpio::Input<'a> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_input(self) -> bouffalo_hal::gpio::Input<'a> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_input(self) -> bouffalo_hal::gpio::Input<'a> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::None)
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::IntoPad<'static> for $Pad<N> {
            #[inline]
            fn into_pull_up_output(self) -> bouffalo_hal::gpio::Output<'static> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_output(self) -> bouffalo_hal::gpio::Output<'static> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_output(self) -> bouffalo_hal::gpio::Output<'static> {
                bouffalo_hal::gpio::Output::new(self, N, bouffalo_hal::glb::Pull::None)
            }
            #[inline]
            fn into_pull_up_input(self) -> bouffalo_hal::gpio::Input<'static> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_input(self) -> bouffalo_hal::gpio::Input<'static> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_input(self) -> bouffalo_hal::gpio::Input<'static> {
                bouffalo_hal::gpio::Input::new(self, N, bouffalo_hal::glb::Pull::None)
            }
        }

        impl<'a, const N: usize> bouffalo_hal::gpio::IntoPadv2<'a, N> for &'a mut $Pad<N> {
            #[inline]
            fn into_spi<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Spi<I>> {
                bouffalo_hal::gpio::Alternate::new_spi::<I>(self)
            }
            #[inline]
            fn into_sdh(self) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Sdh> {
                bouffalo_hal::gpio::Alternate::new_sdh(self)
            }
            #[inline]
            fn into_uart(self) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Uart> {
                bouffalo_hal::gpio::Alternate::new_uart(self)
            }
            #[inline]
            fn into_mm_uart(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::MmUart> {
                bouffalo_hal::gpio::Alternate::new_mm_uart(self)
            }
            #[inline]
            fn into_pull_up_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::None)
            }
            #[inline]
            fn into_i2c<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::I2c<I>> {
                bouffalo_hal::gpio::Alternate::new_i2c(self)
            }
            #[inline]
            fn into_jtag_d0(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::JtagD0> {
                bouffalo_hal::gpio::Alternate::new_jtag_d0(self)
            }
            #[inline]
            fn into_jtag_m0(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::JtagM0> {
                bouffalo_hal::gpio::Alternate::new_jtag_m0(self)
            }
            #[inline]
            fn into_jtag_lp(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'a, N, bouffalo_hal::gpio::JtagLp> {
                bouffalo_hal::gpio::Alternate::new_jtag_lp(self)
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::IntoPadv2<'static, N> for $Pad<N> {
            #[inline]
            fn into_spi<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Spi<I>> {
                bouffalo_hal::gpio::Alternate::new_spi::<I>(self)
            }
            #[inline]
            fn into_sdh(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Sdh> {
                bouffalo_hal::gpio::Alternate::new_sdh(self)
            }
            #[inline]
            fn into_uart(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Uart> {
                bouffalo_hal::gpio::Alternate::new_uart(self)
            }
            #[inline]
            fn into_mm_uart(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::MmUart> {
                bouffalo_hal::gpio::Alternate::new_mm_uart(self)
            }
            #[inline]
            fn into_pull_up_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::Up)
            }
            #[inline]
            fn into_pull_down_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::Down)
            }
            #[inline]
            fn into_floating_pwm<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::Pwm<I>> {
                bouffalo_hal::gpio::Alternate::new_pwm(self, bouffalo_hal::glb::Pull::None)
            }
            #[inline]
            fn into_i2c<const I: usize>(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::I2c<I>> {
                bouffalo_hal::gpio::Alternate::new_i2c(self)
            }
            #[inline]
            fn into_jtag_d0(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::JtagD0> {
                bouffalo_hal::gpio::Alternate::new_jtag_d0(self)
            }
            #[inline]
            fn into_jtag_m0(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::JtagM0> {
                bouffalo_hal::gpio::Alternate::new_jtag_m0(self)
            }
            #[inline]
            fn into_jtag_lp(
                self,
            ) -> bouffalo_hal::gpio::Alternate<'static, N, bouffalo_hal::gpio::JtagLp> {
                bouffalo_hal::gpio::Alternate::new_jtag_lp(self)
            }
        }
    };
}
