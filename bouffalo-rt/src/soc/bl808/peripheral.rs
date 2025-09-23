use bouffalo_hal::{
    dma::{EightChannels, FourChannels, Periph4Dma01, Periph4Dma2},
    gpio::{Alternate, FlexPad},
    i2c::{IntoI2cScl, IntoI2cSda},
    spi::{IntoSpiClk, IntoSpiCs, IntoSpiMiso, IntoSpiMosi},
    uart::IntoUartPad,
};

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Global configuration peripheral.
    pub glb: GLBv2,
    /// General Purpose Input/Output pads.
    pub gpio: Pads,
    /// UART signal multiplexers.
    pub uart_muxes: UartMuxes,
    /// Universal Asynchronous Receiver/Transmitter peripheral 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter peripheral 1.
    pub uart1: UART1,
    /// Serial Peripheral Interface peripheral 0.
    pub spi0: SPI0,
    /// Inter-Integrated Circuit bus peripheral 0.
    pub i2c0: I2C0,
    /// Pulse Width Modulation peripheral.
    pub pwm: PWM,
    /// Inter-Integrated Circuit bus peripheral 1.
    pub i2c1: I2C1,
    /// Universal Asynchronous Receiver/Transmitter peripheral 2.
    pub uart2: UART2,
    /// Hardware LZ4 Decompressor.
    pub lz4d: LZ4D,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Ethernet Media Access Control peripheral.
    pub emac: EMAC,
    /// Universal Asynchronous Receiver/Transmitter peripheral 3.
    pub uart3: UART3,
    /// Inter-Integrated Circuit bus peripheral 2.
    pub i2c2: I2C2,
    /// Inter-Integrated Circuit bus peripheral 3.
    pub i2c3: I2C3,
    /// Serial Peripheral Interface peripheral 1.
    pub spi1: SPI1,
    /// Platform-local Interrupt Controller.
    pub plic: PLIC,
    /// Multi-media subsystem global peripheral.
    pub mmglb: MMGLB,
    /// Pseudo Static Random Access Memory controller.
    pub psram: PSRAM,
    /// Secure Digital High Capacity peripheral.
    pub sdh: SDH,
    /// Direct Memory Access peripheral 0.
    pub dma0: DMA0,
    /// Direct Memory Access peripheral 1.
    pub dma1: DMA1,
    /// Direct Memory Access peripheral 2.
    pub dma2: DMA2,
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub gpip: GPIP,
    /// Efuse peripheral.
    pub efuse: EFUSE,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv2 => 0x20000000, bouffalo_hal::glb::v2::RegisterBlock;
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub struct GPIP => 0x20002000, bouffalo_hal::gpip::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0 with fixed base address.
    pub struct UART0 => 0x2000A000, bouffalo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1 with fixed base address.
    pub struct UART1 => 0x2000A100, bouffalo_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral 0.
    pub struct SPI0 => 0x2000A200, bouffalo_hal::spi::RegisterBlock;
    /// Inter-Integrated Circuit bus 0 with fixed base address.
    pub struct I2C0 => 0x2000A300, bouffalo_hal::i2c::RegisterBlock;
    /// Pulse Width Modulation peripheral.
    pub struct PWM => 0x2000A400, bouffalo_hal::pwm::RegisterBlock;
    /// Inter-Integrated Circuit bus 1 with fixed base address.
    pub struct I2C1 => 0x2000A900, bouffalo_hal::i2c::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 2 with fixed base address.
    pub struct UART2 => 0x2000AA00, bouffalo_hal::uart::RegisterBlock;
    /// Hardware LZ4 Decompressor.
    pub struct LZ4D => 0x2000AD00, bouffalo_hal::lz4d::RegisterBlock;
    /// Direct Memory Access peripheral 0.
    pub struct DMA0 => 0x2000C000, bouffalo_hal::dma::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x2000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Efuse peripheral.
    pub struct EFUSE => 0x20056000, bouffalo_hal::efuse::RegisterBlock;
    /// Secure Digital High Capacity peripheral.
    pub struct SDH => 0x20060000, bouffalo_hal::sdio::RegisterBlock;
    /// Ethernet Media Access Control peripheral.
    pub struct EMAC => 0x20070000, bouffalo_hal::emac::RegisterBlock;
    /// Direct Memory Access peripheral 1.
    pub struct DMA1 => 0x20071000, bouffalo_hal::dma::RegisterBlock;
    /// Direct Memory Access peripheral 2.
    pub struct DMA2 => 0x30001000, bouffalo_hal::dma::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 3 with fixed base address.
    pub struct UART3 => 0x30002000, bouffalo_hal::uart::RegisterBlock;
    /// Inter-Integrated Circuit bus 2 with fixed base address.
    pub struct I2C2 => 0x30003000, bouffalo_hal::i2c::RegisterBlock;
    /// Inter-Integrated Circuit bus 3 with fixed base address.
    pub struct I2C3 => 0x30004000, bouffalo_hal::i2c::RegisterBlock;
    /// Multi-media subsystem global peripheral.
    pub struct MMGLB => 0x30007000, bouffalo_hal::glb::mm::RegisterBlock;
    /// Serial Peripheral Interface peripheral 1.
    pub struct SPI1 => 0x30008000, bouffalo_hal::spi::RegisterBlock;
    /// Pseudo Static Random Access Memory controller.
    pub struct PSRAM => 0x3000F000, bouffalo_hal::psram::RegisterBlock;
    /// Platform-local Interrupt Controller.
    pub struct PLIC => 0xE0000000, xuantie_riscv::peripheral::plic::Plic;
}

dma! {
    DMA0: (0, EightChannels, Periph4Dma01),
    DMA1: (1, FourChannels, Periph4Dma01),
    DMA2: (2, EightChannels, Periph4Dma2),
}

uart! { UART0: 0, UART1: 1, UART2: 2, UART3: 3, }

spi! { SPI0: 0, SPI1: 1, }

i2c! { I2C0: 0, I2C1: 1, I2C2: 2, I2C3: 3, }

pwm! { PWM, }

lz4d! { LZ4D, }

/// BL808 GPIO pad.
pub struct Pad<const N: usize> {
    _private: (),
}

impl_pad_v2! { Pad: GLBv2 }

pad_uart! {
    Pad;
    (0 => 0, 1 => 1, 2 => 2, 3 => 3, 4 => 4, 5 => 5, 6 => 6, 7 => 7, 8 => 8, 9 => 9, 10 => 10, 11 => 11,
    12 => 0, 13 => 1, 14 => 2, 15 => 3, 16 => 4, 17 => 5, 18 => 6, 19 => 7, 20 => 8, 21 => 9, 22 => 10, 23 => 11,
    24 => 0, 25 => 1, 26 => 2, 27 => 3, 28 => 4, 29 => 5, 30 => 6, 31 => 7, 32 => 8, 33 => 9, 34 => 10, 35 => 11,
    36 => 0, 37 => 1, 38 => 2, 39 => 3, 40 => 4, 41 => 5, 42 => 6, 43 => 7, 44 => 8, 45 => 9,): IntoUartPad;
}

pad_spi! {
    Pad;
    (3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43,     ): IntoSpiClk<1>, into_spi_clk;
    (1, 5, 9,  13, 17, 21, 25, 29, 33, 37, 41, 45, ): IntoSpiMosi<1>, into_spi_mosi;
    (2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42,     ): IntoSpiMiso<1>, into_spi_miso;
    (0, 4, 8,  12, 16, 20, 24, 28, 32, 36, 40, 44, ): IntoSpiCs<1>, into_spi_cs;
    (3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43,     ): IntoSpiClk<0>, into_spi_clk;
    (1, 5, 9,  13, 17, 21, 25, 29, 33, 37, 41, 45, ): IntoSpiMosi<0>, into_spi_mosi;
    (2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42,     ): IntoSpiMiso<0>, into_spi_miso;
    (0, 4, 8,  12, 16, 20, 24, 28, 32, 36, 40, 44, ): IntoSpiCs<0>, into_spi_cs;
}

pad_i2c! {
    Pad;
    (0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22,
        24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44,): IntoI2cScl<0>, into_i2c_scl;
    (1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23,
        25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,): IntoI2cSda<0>, into_i2c_sda;
    (0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22,
        24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44,): IntoI2cScl<1>, into_i2c_scl;
    (1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23,
        25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,): IntoI2cSda<1>, into_i2c_sda;
    (0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22,
        24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44,): IntoI2cScl<2>, into_i2c_scl;
    (1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23,
        25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,): IntoI2cSda<2>, into_i2c_sda;
    (0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22,
        24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44,): IntoI2cScl<3>, into_i2c_scl;
    (1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23,
        25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,): IntoI2cSda<3>, into_i2c_sda;
}

/// Available UART signal multiplexers for BL808.
pub struct UartMuxes {
    /// Multiplexer of UART signal 0.
    pub sig0: UartMux<0>,
    /// Multiplexer of UART signal 1.
    pub sig1: UartMux<1>,
    /// Multiplexer of UART signal 2.
    pub sig2: UartMux<2>,
    /// Multiplexer of UART signal 3.
    pub sig3: UartMux<3>,
    /// Multiplexer of UART signal 4.
    pub sig4: UartMux<4>,
    /// Multiplexer of UART signal 5.
    pub sig5: UartMux<5>,
    /// Multiplexer of UART signal 6.
    pub sig6: UartMux<6>,
    /// Multiplexer of UART signal 7.
    pub sig7: UartMux<7>,
    /// Multiplexer of UART signal 8.
    pub sig8: UartMux<8>,
    /// Multiplexer of UART signal 9.
    pub sig9: UartMux<9>,
    /// Multiplexer of UART signal 10.
    pub sig10: UartMux<10>,
    /// Multiplexer of UART signal 11.
    pub sig11: UartMux<11>,
}

// Internal function, do not use.
impl UartMuxes {
    #[inline]
    fn __new() -> Self {
        UartMuxes {
            sig0: UartMux { _private: () },
            sig1: UartMux { _private: () },
            sig2: UartMux { _private: () },
            sig3: UartMux { _private: () },
            sig4: UartMux { _private: () },
            sig5: UartMux { _private: () },
            sig6: UartMux { _private: () },
            sig7: UartMux { _private: () },
            sig8: UartMux { _private: () },
            sig9: UartMux { _private: () },
            sig10: UartMux { _private: () },
            sig11: UartMux { _private: () },
        }
    }
}

/// Global peripheral UART signal multiplexer.
///
/// This structure only owns the signal multiplexer for signal number `I`.
pub struct UartMux<const I: usize> {
    _private: (),
}

impl<const N: usize> bouffalo_hal::uart::IntoUartSignal<'static, N> for UartMux<N> {
    #[inline]
    fn into_transmit<const I: usize>(
        self,
        pad: impl bouffalo_hal::uart::IntoUartPad<'static, N>,
    ) -> bouffalo_hal::uart::Transmit<'static, I> {
        use bouffalo_hal::uart::MuxTxd;
        let glb = &*GLBv2 { _private: () };
        let config = glb.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxTxd::<I>::signal());
        unsafe { glb.uart_mux_group[N >> 3].write(config) };
        bouffalo_hal::uart::Transmit::__new(glb, pad.into_uart_pad())
    }
    #[inline]
    fn into_receive<const I: usize>(
        self,
        pad: impl bouffalo_hal::uart::IntoUartPad<'static, N>,
    ) -> bouffalo_hal::uart::Receive<'static, I> {
        use bouffalo_hal::uart::MuxRxd;
        let glb = &*GLBv2 { _private: () };
        let config = glb.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxRxd::<I>::signal());
        unsafe { glb.uart_mux_group[N >> 3].write(config) };
        bouffalo_hal::uart::Receive::__new(glb, pad.into_uart_pad())
    }
    #[inline]
    fn into_request_to_send<const I: usize>(
        self,
        pad: impl bouffalo_hal::uart::IntoUartPad<'static, N>,
    ) -> bouffalo_hal::uart::RequestToSend<'static, I> {
        use bouffalo_hal::uart::MuxRts;
        let glb = &*GLBv2 { _private: () };
        let config = glb.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxRts::<I>::signal());
        unsafe { glb.uart_mux_group[N >> 3].write(config) };
        bouffalo_hal::uart::RequestToSend::__new(glb, pad.into_uart_pad())
    }
    #[inline]
    fn into_clear_to_send<const I: usize>(
        self,
        pad: impl bouffalo_hal::uart::IntoUartPad<'static, N>,
    ) -> bouffalo_hal::uart::ClearToSend<'static, I> {
        use bouffalo_hal::uart::MuxCts;
        let glb = &*GLBv2 { _private: () };
        let config = glb.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxCts::<I>::signal());
        unsafe { glb.uart_mux_group[N >> 3].write(config) };
        bouffalo_hal::uart::ClearToSend::__new(glb, pad.into_uart_pad())
    }
}

/// Available GPIO pads for BL808.
pub struct Pads {
    /// GPIO I/O 0.
    pub io0: Pad<0>,
    /// GPIO I/O 1.
    pub io1: Pad<1>,
    /// GPIO I/O 2.
    pub io2: Pad<2>,
    /// GPIO I/O 3.
    pub io3: Pad<3>,
    /// GPIO I/O 4.
    pub io4: Pad<4>,
    /// GPIO I/O 5.
    pub io5: Pad<5>,
    /// GPIO I/O 6.
    pub io6: Pad<6>,
    /// GPIO I/O 7.
    pub io7: Pad<7>,
    /// GPIO I/O 8.
    pub io8: Pad<8>,
    /// GPIO I/O 9.
    pub io9: Pad<9>,
    /// GPIO I/O 10.
    pub io10: Pad<10>,
    /// GPIO I/O 11.
    pub io11: Pad<11>,
    /// GPIO I/O 12.
    pub io12: Pad<12>,
    /// GPIO I/O 13.
    pub io13: Pad<13>,
    /// GPIO I/O 14.
    pub io14: Pad<14>,
    /// GPIO I/O 15.
    pub io15: Pad<15>,
    /// GPIO I/O 16.
    pub io16: Pad<16>,
    /// GPIO I/O 17.
    pub io17: Pad<17>,
    /// GPIO I/O 18.
    pub io18: Pad<18>,
    /// GPIO I/O 19.
    pub io19: Pad<19>,
    /// GPIO I/O 20.
    pub io20: Pad<20>,
    /// GPIO I/O 21.
    pub io21: Pad<21>,
    /// GPIO I/O 22.
    pub io22: Pad<22>,
    /// GPIO I/O 23.
    pub io23: Pad<23>,
    /// GPIO I/O 24.
    pub io24: Pad<24>,
    /// GPIO I/O 25.
    pub io25: Pad<25>,
    /// GPIO I/O 26.
    pub io26: Pad<26>,
    /// GPIO I/O 27.
    pub io27: Pad<27>,
    /// GPIO I/O 28.
    pub io28: Pad<28>,
    /// GPIO I/O 29.
    pub io29: Pad<29>,
    /// GPIO I/O 30.
    pub io30: Pad<30>,
    /// GPIO I/O 31.
    pub io31: Pad<31>,
    /// GPIO I/O 32.
    pub io32: Pad<32>,
    /// GPIO I/O 33.
    pub io33: Pad<33>,
    /// GPIO I/O 34.
    pub io34: Pad<34>,
    /// GPIO I/O 35.
    pub io35: Pad<35>,
    /// GPIO I/O 36.
    pub io36: Pad<36>,
    /// GPIO I/O 37.
    pub io37: Pad<37>,
    /// GPIO I/O 38.
    pub io38: Pad<38>,
    /// GPIO I/O 39.
    pub io39: Pad<39>,
    /// GPIO I/O 40.
    pub io40: Pad<40>,
    /// GPIO I/O 41.
    pub io41: Pad<41>,
    /// GPIO I/O 42.
    pub io42: Pad<42>,
    /// GPIO I/O 43.
    pub io43: Pad<43>,
    /// GPIO I/O 44.
    pub io44: Pad<44>,
    /// GPIO I/O 45.
    pub io45: Pad<45>,
}

// Internal function, do not use.
impl Pads {
    #[inline]
    fn __new() -> Self {
        Pads {
            io0: Pad { _private: () },
            io1: Pad { _private: () },
            io2: Pad { _private: () },
            io3: Pad { _private: () },
            io4: Pad { _private: () },
            io5: Pad { _private: () },
            io6: Pad { _private: () },
            io7: Pad { _private: () },
            io8: Pad { _private: () },
            io9: Pad { _private: () },
            io10: Pad { _private: () },
            io11: Pad { _private: () },
            io12: Pad { _private: () },
            io13: Pad { _private: () },
            io14: Pad { _private: () },
            io15: Pad { _private: () },
            io16: Pad { _private: () },
            io17: Pad { _private: () },
            io18: Pad { _private: () },
            io19: Pad { _private: () },
            io20: Pad { _private: () },
            io21: Pad { _private: () },
            io22: Pad { _private: () },
            io23: Pad { _private: () },
            io24: Pad { _private: () },
            io25: Pad { _private: () },
            io26: Pad { _private: () },
            io27: Pad { _private: () },
            io28: Pad { _private: () },
            io29: Pad { _private: () },
            io30: Pad { _private: () },
            io31: Pad { _private: () },
            io32: Pad { _private: () },
            io33: Pad { _private: () },
            io34: Pad { _private: () },
            io35: Pad { _private: () },
            io36: Pad { _private: () },
            io37: Pad { _private: () },
            io38: Pad { _private: () },
            io39: Pad { _private: () },
            io40: Pad { _private: () },
            io41: Pad { _private: () },
            io42: Pad { _private: () },
            io43: Pad { _private: () },
            io44: Pad { _private: () },
            io45: Pad { _private: () },
        }
    }
}

impl Peripherals {
    #[inline]
    pub(crate) fn __new() -> Self {
        Self {
            glb: GLBv2 { _private: () },
            gpio: Pads::__new(),
            uart_muxes: UartMuxes::__new(),
            uart0: UART0 { _private: () },
            uart1: UART1 { _private: () },
            spi0: SPI0 { _private: () },
            i2c0: I2C0 { _private: () },
            pwm: PWM { _private: () },
            i2c1: I2C1 { _private: () },
            uart2: UART2 { _private: () },
            lz4d: LZ4D { _private: () },
            hbn: HBN { _private: () },
            emac: EMAC { _private: () },
            uart3: UART3 { _private: () },
            i2c2: I2C2 { _private: () },
            i2c3: I2C3 { _private: () },
            spi1: SPI1 { _private: () },
            plic: PLIC { _private: () },
            mmglb: MMGLB { _private: () },
            psram: PSRAM { _private: () },
            sdh: SDH { _private: () },
            dma0: DMA0 { _private: () },
            dma1: DMA1 { _private: () },
            dma2: DMA2 { _private: () },
            gpip: GPIP { _private: () },
            efuse: EFUSE { _private: () },
        }
    }
}
