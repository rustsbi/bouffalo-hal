/// Peripherals available on ROM start.
pub struct Peripherals<'a> {
    /// Global configuration peripheral.
    pub glb: GLBv2,
    /// General Purpose Input/Output pads.
    pub gpio: bouffalo_hal::gpio::Pads<'a>,
    /// UART signal multiplexers.
    pub uart_muxes: bouffalo_hal::uart::UartMuxes<'a>,
    /// Universal Asynchronous Receiver/Transmitter peripheral 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter peripheral 1.
    pub uart1: UART1,
    /// Serial Peripheral Interface peripheral.
    pub spi: SPI,
    /// Inter-Integrated Circuit bus peripheral 0.
    pub i2c0: I2C0,
    /// Pulse Width Modulation peripheral.
    pub pwm: PWM,
    /// Inter-Integrated Circuit bus peripheral 1.
    pub i2c1: I2C1,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Ethernet Media Access Control peripheral.
    pub emac: EMAC,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv2 => 0x20000000, bouffalo_hal::glb::v2::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0 with fixed base address.
    pub struct UART0 => 0x2000A000, bouffalo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1 with fixed base address.
    pub struct UART1 => 0x2000A100, bouffalo_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral.
    pub struct SPI => 0x2000A200, bouffalo_hal::spi::RegisterBlock;
    /// Inter-Integrated Circuit bus 0 with fixed base address.
    pub struct I2C0 => 0x2000A300, bouffalo_hal::i2c::RegisterBlock;
    /// Pulse Width Modulation peripheral.
    pub struct PWM => 0x2000A400, bouffalo_hal::pwm::RegisterBlock;
    /// Inter-Integrated Circuit bus 1 with fixed base address.
    pub struct I2C1 => 0x2000A900, bouffalo_hal::i2c::RegisterBlock;
   /// Hibernation control peripheral.
    pub struct HBN => 0x2000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Ethernet Media Access Control peripheral.
    pub struct EMAC => 0x20070000, bouffalo_hal::emac::RegisterBlock;
}

pub use bouffalo_hal::clocks::Clocks;

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params(xtal_hz: u32) -> (Peripherals<'static>, Clocks) {
    use embedded_time::rate::Hertz;
    let peripherals = Peripherals {
        glb: GLBv2 { _private: () },
        gpio: match () {
            #[cfg(feature = "bl616")]
            () => bouffalo_hal::gpio::Pads::__pads_from_glb(&GLBv2 { _private: () }),
            #[cfg(not(feature = "bl616"))]
            () => unimplemented!(),
        },
        uart_muxes: bouffalo_hal::uart::UartMuxes::__uart_muxes_from_glb(&GLBv2 { _private: () }),
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        spi: SPI { _private: () },
        i2c0: I2C0 { _private: () },
        pwm: PWM { _private: () },
        i2c1: I2C1 { _private: () },
        hbn: HBN { _private: () },
        emac: EMAC { _private: () },
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}
