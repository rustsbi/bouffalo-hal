pub use bouffalo_hal::clocks::Clocks;

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Global configuration peripheral.
    pub glb: GLBv1,
    /// Universal Asynchronous Receiver/Transmitter peripheral 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver/Transmitter peripheral 1.
    pub uart1: UART1,
    /// Serial Peripheral Interface peripheral.
    pub spi: SPI,
    /// Inter-Integrated Circuit bus peripheral.
    pub i2c: I2C,
    /// Pulse Width Modulation peripheral.
    pub pwm: PWM,
    /// Ethernet Media Access Control peripheral.
    pub emac: EMAC,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Universal Serial Bus peripheral.
    pub usb: USBv1,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv1 => 0x40000000, bouffalo_hal::glb::v1::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0 with fixed base address.
    pub struct UART0 => 0x4000A000, bouffalo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1 with fixed base address.
    pub struct UART1 => 0x4000A100, bouffalo_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral.
    pub struct SPI => 0x4000A200, bouffalo_hal::spi::RegisterBlock;
    /// Inter-Integrated Circuit bus with fixed base address.
    pub struct I2C => 0x4000A300, bouffalo_hal::i2c::RegisterBlock;
    /// Pulse Width Modulation peripheral.
    pub struct PWM => 0x4000A400, bouffalo_hal::pwm::RegisterBlock;
    /// Ethernet Media Access Control peripheral.
    pub struct EMAC => 0x4000D000, bouffalo_hal::emac::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x4000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Universal Serial Bus peripheral.
    pub struct USBv1 => 0x4000D800, bouffalo_hal::usb::v1::RegisterBlock;
}

// TODO: BL702 clock tree configuration.
// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params(xtal_hz: u32) -> (Peripherals, Clocks) {
    use embedded_time::rate::Hertz;
    let peripherals = Peripherals {
        glb: GLBv1 { _private: () },
        uart0: UART0 { _private: () },
        uart1: UART1 { _private: () },
        spi: SPI { _private: () },
        i2c: I2C { _private: () },
        pwm: PWM { _private: () },
        emac: EMAC { _private: () },
        hbn: HBN { _private: () },
        usb: USBv1 { _private: () },
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}
