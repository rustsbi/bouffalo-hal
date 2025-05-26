pub use bouffalo_hal::clocks::Clocks;

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Global configuration peripheral.
    pub glb: GLBv1,
    /// General Purpose Input/Output pads.
    pub gpio: Pads,
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

uart! {
    UART0: 0,
    UART1: 1,
}

spi! { SPI, }

pwm! { PWM, }

/// BL702 GPIO pad.
pub struct Pad<const N: usize> {
    _private: (),
}

impl_pad_v1! { Pad: GLBv1 }

/// Available GPIO pads for BL702.
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
        }
    }
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
        gpio: Pads::__new(),
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
