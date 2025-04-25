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
    /// Universal Asynchronous Receiver/Transmitter 2 with fixed base address.
    pub struct UART2 => 0x2000AA00, bouffalo_hal::uart::RegisterBlock;
    /// Hardware LZ4 Decompressor.
    pub struct LZ4D => 0x2000AD00, bouffalo_hal::lz4d::RegisterBlock;
    /// Direct Memory Access peripheral 0.
    pub struct DMA0 => 0x2000C000, bouffalo_hal::dma::RegisterBlock;
    /// Hibernation control peripheral.
    pub struct HBN => 0x2000F000, bouffalo_hal::hbn::RegisterBlock;
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

pub use bouffalo_hal::clocks::Clocks;
use bouffalo_hal::dma::{EightChannels, FourChannels, Periph4Dma01, Periph4Dma2};

dma! {
    DMA0: (0, EightChannels, Periph4Dma01),
    DMA1: (1, FourChannels, Periph4Dma01),
    DMA2: (2, EightChannels, Periph4Dma2),
}

uart! {
    UART0: 0,
    UART1: 1,
    UART2: 2,
    UART3: 3,
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params(xtal_hz: u32) -> (Peripherals<'static>, Clocks) {
    use embedded_time::rate::Hertz;
    let peripherals = Peripherals {
        glb: GLBv2 { _private: () },
        gpio: match () {
            #[cfg(any(feature = "bl808-dsp", feature = "bl808-mcu", feature = "bl808-lp"))]
            () => bouffalo_hal::gpio::Pads::__pads_from_glb(&GLBv2 { _private: () }),
            #[cfg(not(any(feature = "bl808-dsp", feature = "bl808-mcu", feature = "bl808-lp")))]
            () => unimplemented!(),
        },
        uart_muxes: bouffalo_hal::uart::UartMuxes::__uart_muxes_from_glb(&GLBv2 { _private: () }),
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
    };
    let clocks = Clocks {
        xtal: Hertz(xtal_hz),
    };
    (peripherals, clocks)
}
