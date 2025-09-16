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
    impl bouffalo_hal::uart::Numbered<'static, $i> for $UARTx {}

    impl<'a> bouffalo_hal::uart::Instance<'a> for &'a mut $UARTx {
        #[inline]
        fn register_block(self) -> &'a bouffalo_hal::uart::RegisterBlock {
            &*self
        }
    }
    impl<'a> bouffalo_hal::uart::Numbered<'a, $i> for &'a mut $UARTx {}

    impl<'a> bouffalo_hal::uart::UartExt<'a, $i> for &'a mut $UARTx {
        #[inline]
        fn freerun(
            self,
            config: bouffalo_hal::uart::Config,
            pads: impl bouffalo_hal::uart::IntoSignals<'a, $i>,
            clocks: &Clocks,
        ) -> Result<bouffalo_hal::uart::BlockingSerial<'a>, bouffalo_hal::uart::ConfigError> {
            bouffalo_hal::uart::BlockingSerial::new_freerun(self, config, pads, clocks)
        }
        #[inline]
        fn with_interrupt(
            self,
            config: bouffalo_hal::uart::Config,
            pads: impl bouffalo_hal::uart::IntoSignals<'a, $i>,
            clocks: &Clocks,
            state: &'static bouffalo_hal::uart::SerialState,
        ) -> Result<bouffalo_hal::uart::AsyncSerial<'a>, bouffalo_hal::uart::ConfigError> {
            bouffalo_hal::uart::AsyncSerial::new(self, config, pads, clocks, state)
        }
    }

    impl bouffalo_hal::uart::UartExt<'static, $i> for $UARTx {
        #[inline]
        fn freerun(
            self,
            config: bouffalo_hal::uart::Config,
            pads: impl bouffalo_hal::uart::IntoSignals<'static, $i>,
            clocks: &Clocks,
        ) -> Result<bouffalo_hal::uart::BlockingSerial<'static>, bouffalo_hal::uart::ConfigError> {
            bouffalo_hal::uart::BlockingSerial::new_freerun(self, config, pads, clocks)
        }
        #[inline]
        fn with_interrupt(
            self,
            config: bouffalo_hal::uart::Config,
            pads: impl bouffalo_hal::uart::IntoSignals<'static, $i>,
            clocks: &Clocks,
            state: &'static bouffalo_hal::uart::SerialState,
        ) -> Result<bouffalo_hal::uart::AsyncSerial<'static>, bouffalo_hal::uart::ConfigError> {
            bouffalo_hal::uart::AsyncSerial::new(self, config, pads, clocks, state)
        }
    }
)+
    };
}

macro_rules! spi {
    ($($SPIx: ty: $i: expr,)+) => {
    $(
impl bouffalo_hal::spi::Instance<'static> for $SPIx {
    #[inline]
    fn register_block(self) -> &'static bouffalo_hal::spi::RegisterBlock {
        unsafe { &*Self::ptr() }
    }
}
impl bouffalo_hal::spi::Numbered<'static, $i> for $SPIx {}

impl<'a> bouffalo_hal::spi::Instance<'a> for &'a mut $SPIx {
    #[inline]
    fn register_block(self) -> &'a bouffalo_hal::spi::RegisterBlock {
        &*self
    }
}
impl<'a> bouffalo_hal::spi::Numbered<'a, $i> for &'a mut $SPIx {}
    )+
    };
}

macro_rules! i2c {
    ($($I2Ci: ty: $i: expr,)+) => {
    $(
impl bouffalo_hal::i2c::Instance<'static> for $I2Ci {
    #[inline]
    fn register_block(self) -> &'static bouffalo_hal::i2c::RegisterBlock {
        unsafe { &*Self::ptr() }
    }
}
impl bouffalo_hal::i2c::Numbered<'static, $i> for $I2Ci {}

impl<'a> bouffalo_hal::i2c::Instance<'a> for &'a mut $I2Ci {
    #[inline]
    fn register_block(self) -> &'a bouffalo_hal::i2c::RegisterBlock {
        &*self
    }
}
impl<'a> bouffalo_hal::i2c::Numbered<'a, $i> for &'a mut $I2Ci {}
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

macro_rules! lz4d {
    ($($LZ4Dx: ty,)+) => {
    $(
impl bouffalo_hal::lz4d::Instance<'static> for $LZ4Dx {
    #[inline]
    fn register_block(self) -> &'static bouffalo_hal::lz4d::RegisterBlock {
        unsafe { &*Self::ptr() }
    }
}

impl<'a> bouffalo_hal::lz4d::Instance<'a> for &'a mut $LZ4Dx {
    #[inline]
    fn register_block(self) -> &'a bouffalo_hal::lz4d::RegisterBlock {
        &*self
    }
}

impl bouffalo_hal::lz4d::Lz4dExt<'static> for $LZ4Dx {
    #[inline]
    fn decompress<R, W>(self, input: core::pin::Pin<R>, output: core::pin::Pin<W>)
        -> bouffalo_hal::lz4d::Decompress<'static, R, W>
    where
        R: core::ops::Deref + 'static,
        R::Target: as_slice::AsSlice<Element = u8>,
        W: core::ops::DerefMut + 'static,
        W::Target: as_slice::AsMutSlice<Element = u8>,
    {
        bouffalo_hal::lz4d::Decompress::new(self, input, output)
    }
}

impl<'a> bouffalo_hal::lz4d::Lz4dExt<'a> for &'a mut $LZ4Dx {
    /// Create and start an LZ4D decompression request.
    #[inline]
    fn decompress<R, W>(self, input: core::pin::Pin<R>, output: core::pin::Pin<W>)
        -> bouffalo_hal::lz4d::Decompress<'a, R, W>
    where
        R: core::ops::Deref + 'static,
        R::Target: as_slice::AsSlice<Element = u8>,
        W: core::ops::DerefMut + 'static,
        W::Target: as_slice::AsMutSlice<Element = u8>,
    {
        bouffalo_hal::lz4d::Decompress::new(self, input, output)
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

macro_rules! impl_pad_v1 {
    ($Pad: ident: $GLBv1: ident) => {
        impl<'a, const N: usize> bouffalo_hal::gpio::Instance<'a> for &'a mut $Pad<N> {
            #[inline]
            fn register_block(self) -> &'a bouffalo_hal::gpio::AnyRegisterBlock {
                <&bouffalo_hal::gpio::AnyRegisterBlock as From<
                    &bouffalo_hal::glb::v1::RegisterBlock,
                >>::from(&$GLBv1 { _private: () })
            }
            #[inline]
            fn version(&self) -> bouffalo_hal::glb::Version {
                bouffalo_hal::glb::Version::V1
            }
        }

        impl<'a, const N: usize> bouffalo_hal::gpio::Numbered<'a, N> for &'a mut $Pad<N> {}

        impl<const N: usize> bouffalo_hal::gpio::Instance<'static> for $Pad<N> {
            #[inline]
            fn register_block(self) -> &'static bouffalo_hal::gpio::AnyRegisterBlock {
                <&bouffalo_hal::gpio::AnyRegisterBlock as From<
                    &bouffalo_hal::glb::v1::RegisterBlock,
                >>::from(&$GLBv1 { _private: () })
            }
            #[inline]
            fn version(&self) -> bouffalo_hal::glb::Version {
                bouffalo_hal::glb::Version::V1
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::Numbered<'static, N> for $Pad<N> {}

        impl<'a, const N: usize> bouffalo_hal::gpio::pad_v1::Instance<'a> for &'a mut $Pad<N> {
            #[inline]
            fn register_block(self) -> &'a bouffalo_hal::glb::v1::RegisterBlock {
                &$GLBv1 { _private: () }
            }
        }

        impl<'a, const N: usize> bouffalo_hal::gpio::pad_v1::Numbered<'a, N> for &'a mut $Pad<N> {}

        impl<const N: usize> bouffalo_hal::gpio::pad_v1::Instance<'static> for $Pad<N> {
            #[inline]
            fn register_block(self) -> &'static bouffalo_hal::glb::v1::RegisterBlock {
                &$GLBv1 { _private: () }
            }
        }

        impl<const N: usize> bouffalo_hal::gpio::pad_v1::Numbered<'static, N> for $Pad<N> {}

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
    };
}

macro_rules! pad_spi {
    (
        $Pad: ident;
        $(
            ($($N_pad: expr,)+): $Trait: ident<$I_spi: literal>, $into_spi_signal: ident;
        )+
    ) => {
$($(
impl $Trait<'static, $I_spi> for $Pad<$N_pad> {
    #[inline]
    fn $into_spi_signal(self) -> FlexPad<'static> {
        FlexPad::from_spi(Alternate::new_spi::<$I_spi>(self))
    }
}

impl<'a> $Trait<'a, $I_spi> for &'a mut $Pad<$N_pad> {
    #[inline]
    fn $into_spi_signal(self) -> FlexPad<'a> {
        FlexPad::from_spi(Alternate::new_spi::<$I_spi>(self))
    }
}
)+)+
    };
}

macro_rules! pad_i2c {
    (
        $Pad: ident;
        $(
            ($($N_pad: expr,)+): $Trait: ident<$I_i2c: literal>, $into_i2c_signal: ident;
        )+
    ) => {
$($(
impl $Trait<'static, $I_i2c> for $Pad<$N_pad> {
    #[inline]
    fn $into_i2c_signal(self) -> FlexPad<'static> {
        FlexPad::from_i2c(Alternate::new_i2c::<$I_i2c>(self))
    }
}

impl<'a> $Trait<'a, $I_i2c> for &'a mut $Pad<$N_pad> {
    #[inline]
    fn $into_i2c_signal(self) -> FlexPad<'a> {
        FlexPad::from_i2c(Alternate::new_i2c::<$I_i2c>(self))
    }
}
)+)+
    };
}

macro_rules! pad_uart {
    (
        $Pad: ident;
        $(
            ($($N_pad: expr => $I_signal: expr,)+): $Trait: ident;
        )+
    ) => {
$($(
impl $Trait<'static, $I_signal> for $Pad<$N_pad> {
    #[inline]
    fn into_uart_pad(self) -> FlexPad<'static> {
        FlexPad::from_uart(Alternate::new_uart(self))
    }
}

impl<'a> $Trait<'a, $I_signal> for &'a mut $Pad<$N_pad> {
    #[inline]
    fn into_uart_pad(self) -> FlexPad<'a> {
        FlexPad::from_uart(Alternate::new_uart(self))
    }
}
)+)+
    };
}
