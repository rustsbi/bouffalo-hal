pub use bouffalo_hal::clocks::Clocks;
use bouffalo_hal::{
    gpio::{Alternate, FlexPad},
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
    /// Timer peripheral.
    pub timer: TIMER,
    /// Infrared remote control peripheral.
    pub ir: IR,
    /// DBI (Display Bus Interface) peripheral.
    pub dbi: DBI,
    /// I2S audio peripheral.
    pub i2s: I2S,
    /// Audio ADC peripheral.
    pub auadc: AUADC,
    /// Direct Memory Access peripheral 0.
    pub dma0: DMA0,
    /// Pseudo Static Random Access Memory controller.
    pub psram: PSRAM,
    /// Audio DAC peripheral.
    pub audac: AUDAC,
    /// eFuse peripheral.
    pub efuse: EFUSE,
    /// Secure Digital High Capacity peripheral.
    pub sdh: SDH,
    /// USB peripheral.
    pub usb: USBv1,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Ethernet Media Access Control peripheral.
    pub emac: EMAC,
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub gpip: GPIP,
    /// Hardware LZ4 Decompressor.
    pub lz4d: LZ4D,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv2 => 0x20000000, bouffalo_hal::glb::v2::RegisterBlock;
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
    /// Timer peripheral.
    pub struct TIMER => 0x2000A500, bouffalo_hal::timer::RegisterBlock;
    /// Infrared remote control peripheral.
    pub struct IR => 0x2000A600, bouffalo_hal::ir::RegisterBlock;
    /// DBI (Display Bus Interface) peripheral.
    pub struct DBI => 0x2000A800, bouffalo_hal::dbi::RegisterBlock;
    /// I2S audio peripheral.
    pub struct I2S => 0x2000AB00, bouffalo_hal::i2s::RegisterBlock;
    /// Audio ADC peripheral.
    pub struct AUADC => 0x2000AC00, bouffalo_hal::audio::auadc::RegisterBlock;
    /// Direct Memory Access peripheral 0.
    pub struct DMA0 => 0x2000C000, bouffalo_hal::dma::RegisterBlock;
    /// Hardware LZ4 Decompressor.
    pub struct LZ4D => 0x2000AD00, bouffalo_hal::lz4d::RegisterBlock;
    /// Pseudo Static Random Access Memory controller.
    pub struct PSRAM => 0x20052000, bouffalo_hal::psram::RegisterBlock;
    /// Audio DAC peripheral.
    pub struct AUDAC => 0x20055000, bouffalo_hal::audio::audac::RegisterBlock;
    /// eFuse peripheral.
    pub struct EFUSE => 0x20056000, bouffalo_hal::efuse::RegisterBlock;
    /// Secure Digital High Capacity peripheral.
    pub struct SDH => 0x20060000, bouffalo_hal::sdio::RegisterBlock;
    /// USB peripheral.
    pub struct USBv1 => 0x20072000, bouffalo_hal::usb::v1::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x2000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Ethernet Media Access Control peripheral.
    pub struct EMAC => 0x20070000, bouffalo_hal::emac::RegisterBlock;
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub struct GPIP => 0x20002000, bouffalo_hal::gpip::RegisterBlock;
}

uart! { UART0: 0, UART1: 1, }

spi! { SPI0: 0, }

i2c! { I2C0: 0, I2C1: 1, }

pwm! { PWM, }

/// BL616 GPIO pad.
pub struct Pad<const N: usize> {
    _private: (),
}

impl_pad_v2! { Pad: GLBv2 }

pad_uart! {
    Pad;
    (0 => 0, 1 => 1, 2 => 2, 3 => 3, 4 => 4, 5 => 5, 6 => 6, 7 => 7, 8 => 8, 9 => 9, 10 => 10, 11 => 11,
    12 => 0, 13 => 1, 14 => 2, 15 => 3, 16 => 4, 17 => 5, 18 => 6, 19 => 7, 20 => 8, 21 => 9, 22 => 10, 23 => 11,
    24 => 0, 25 => 1, 26 => 2, 27 => 3, 28 => 4, 29 => 5, 30 => 6, 31 => 7, 32 => 8, 33 => 9, 34 => 10,): IntoUartPad;
}

/// Available GPIO pads for BL616.
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
        }
    }
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params(xtal_hz: u32) -> (Peripherals, Clocks) {
    use embedded_time::rate::Hertz;
    let peripherals = Peripherals {
        glb: GLBv2 { _private: () },
        gpio: Pads::__new(),
        uart_muxes: UartMuxes::__new(),
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        spi0: SPI0 { _private: () },
        i2c0: I2C0 { _private: () },
        pwm: PWM { _private: () },
        i2c1: I2C1 { _private: () },
        timer: TIMER { _private: () },
        ir: IR { _private: () },
        dbi: DBI { _private: () },
        i2s: I2S { _private: () },
        auadc: AUADC { _private: () },
        dma0: DMA0 { _private: () },
        psram: PSRAM { _private: () },
        audac: AUDAC { _private: () },
        efuse: EFUSE { _private: () },
        sdh: SDH { _private: () },
        usb: USBv1 { _private: () },
        hbn: HBN { _private: () },
        emac: EMAC { _private: () },
        gpip: GPIP { _private: () },
        lz4d: LZ4D { _private: () },
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}

/// Available UART signal multiplexers for BL616.
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
