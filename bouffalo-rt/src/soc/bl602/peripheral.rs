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
    /// Serial Peripheral Interface peripheral 0.
    pub spi0: SPI0,
    /// Inter-Integrated Circuit bus peripheral 0.
    pub i2c0: I2C0,
    /// Pulse Width Modulation peripheral.
    pub pwm: PWM,
    /// Timer peripheral.
    pub timer: TIMER,
    /// Infrared remote control peripheral.
    pub ir: IR,
    /// Direct Memory Access peripheral 0.
    pub dma0: DMA0,
    /// Hibernation control peripheral.
    pub hbn: HBN,
    /// Efuse peripheral.
    pub efuse: EFUSE,
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub gpip: GPIP,
}

soc! {
    /// Global configuration peripheral.
    pub struct GLBv1 => 0x40000000, bouffalo_hal::glb::v1::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 0 with fixed base address.
    pub struct UART0 => 0x4000A000, bouffalo_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver/Transmitter 1 with fixed base address.
    pub struct UART1 => 0x4000A100, bouffalo_hal::uart::RegisterBlock;
    /// Serial Peripheral Interface peripheral 0.
    pub struct SPI0 => 0x4000A200, bouffalo_hal::spi::RegisterBlock;
    /// Inter-Integrated Circuit bus 0 with fixed base address.
    pub struct I2C0 => 0x4000A300, bouffalo_hal::i2c::RegisterBlock;
    /// Pulse Width Modulation peripheral.
    pub struct PWM => 0x4000A400, bouffalo_hal::pwm::RegisterBlock;
    /// Timer peripheral.
    pub struct TIMER => 0x4000A500, bouffalo_hal::timer::RegisterBlock;
    /// Infrared remote control peripheral.
    pub struct IR => 0x4000A600, bouffalo_hal::ir::RegisterBlock;
    /// Direct Memory Access peripheral 0.
    pub struct DMA0 => 0x4000C000, bouffalo_hal::dma::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x4000F000, bouffalo_hal::hbn::RegisterBlock;
    /// Efuse peripheral.
    pub struct EFUSE => 0x40007000, bouffalo_hal::efuse::RegisterBlock;
    /// Generic DAC, ADC and ACOMP interface control peripheral.
    pub struct GPIP => 0x40002000, bouffalo_hal::gpip::RegisterBlock;
}

uart! { UART0: 0, UART1: 1, }

spi! { SPI0: 0, }

i2c! { I2C0: 0, }

pwm! { PWM, }

/// BL602 GPIO pad.
pub struct Pad<const N: usize> {
    _private: (),
}

impl_pad_v1! { Pad: GLBv1 }

/// Available GPIO pads for BL602.
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
        }
    }
}

// TODO: BL602 clock tree configuration.
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
        spi0: SPI0 { _private: () },
        i2c0: I2C0 { _private: () },
        pwm: PWM { _private: () },
        timer: TIMER { _private: () },
        ir: IR { _private: () },
        dma0: DMA0 { _private: () },
        hbn: HBN { _private: () },
        efuse: EFUSE { _private: () },
        gpip: GPIP { _private: () },
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}
