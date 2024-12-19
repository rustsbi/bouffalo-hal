use super::{
    typestate::{
        Floating, I2c, Input, JtagD0, JtagLp, JtagM0, MmUart, Output, PullDown, PullUp, Pwm, Sdh,
        Uart,
    },
    Spi,
};
use crate::glb::{v2, Drive, Pull};
use core::{marker::PhantomData, ops::Deref};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// Raw GPIO pad of BL808 and BL616.
pub struct Padv2<GLB, const N: usize, M> {
    base: GLB,
    _mode: PhantomData<M>,
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> Padv2<GLB, N, Input<M>> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        let config = self.base.gpio_config[N].read().enable_schmitt();
        unsafe { self.base.gpio_config[N].write(config) };
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        let config = self.base.gpio_config[N].read().disable_schmitt();
        unsafe { self.base.gpio_config[N].write(config) };
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().clear_interrupt();
        unsafe { self.base.gpio_config[N].write(config) };
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        self.base.gpio_config[N].read().has_interrupt()
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().mask_interrupt();
        unsafe { self.base.gpio_config[N].write(config) };
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().unmask_interrupt();
        unsafe { self.base.gpio_config[N].write(config) };
    }
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> Padv2<GLB, N, Output<M>> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.base.gpio_config[N].read().drive()
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        let config = self.base.gpio_config[N].read().set_drive(val);
        unsafe { self.base.gpio_config[N].write(config) };
    }
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> Padv2<GLB, N, Input<M>> {
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> v2::InterruptMode {
        self.base.gpio_config[N].read().interrupt_mode()
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: v2::InterruptMode) {
        let config = self.base.gpio_config[N].read().set_interrupt_mode(val);
        unsafe { self.base.gpio_config[N].write(config) };
    }
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> Padv2<GLB, N, M> {
    /// Configures the pin to operate as a pull up output pin.
    #[inline]
    pub fn into_pull_up_output(self) -> Padv2<GLB, N, Output<PullUp>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::Up);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull down output pin.
    #[inline]
    pub fn into_pull_down_output(self) -> Padv2<GLB, N, Output<PullDown>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::Down);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a floating output pin.
    #[inline]
    pub fn into_floating_output(self) -> Padv2<GLB, N, Output<Floating>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::None);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull up input pin.
    #[inline]
    pub fn into_pull_up_input(self) -> Padv2<GLB, N, Input<PullUp>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::Up);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull down input pin.
    #[inline]
    pub fn into_pull_down_input(self) -> Padv2<GLB, N, Input<PullDown>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::Down);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a floating input pin.
    #[inline]
    pub fn into_floating_input(self) -> Padv2<GLB, N, Input<Floating>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::None);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
}

const UART_GPIO_CONFIG: v2::GpioConfig = v2::GpioConfig::RESET_VALUE
    .enable_input()
    .enable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::Up)
    .set_function(v2::Function::Uart);
const JTAG_GPIO_CONFIG: v2::GpioConfig = v2::GpioConfig::RESET_VALUE
    .enable_input()
    .disable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::None);

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> Padv2<GLB, N, M> {
    /// Configures the pin to operate as UART signal.
    #[inline]
    pub fn into_uart(self) -> Padv2<GLB, N, Uart> {
        unsafe { self.base.gpio_config[N].write(UART_GPIO_CONFIG) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as multi-media cluster UART signal.
    #[inline]
    pub fn into_mm_uart(self) -> Padv2<GLB, N, MmUart> {
        unsafe {
            self.base.gpio_config[N].write(UART_GPIO_CONFIG.set_function(v2::Function::MmUart))
        };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull up Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_pull_up_pwm<const I: usize>(self) -> Padv2<GLB, N, Pwm<I>> {
        let config = v2::GpioConfig::RESET_VALUE
            .disable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(Pull::Up)
            .set_function(Pwm::<I>::FUNCTION_V2);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull down Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_pull_down_pwm<const I: usize>(self) -> Padv2<GLB, N, Pwm<I>> {
        let config = v2::GpioConfig::RESET_VALUE
            .disable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(Pull::Down)
            .set_function(Pwm::<I>::FUNCTION_V2);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as floating Pulse Width Modulation signal pin.
    #[inline]
    pub fn into_floating_pwm<const I: usize>(self) -> Padv2<GLB, N, Pwm<I>> {
        let config = v2::GpioConfig::RESET_VALUE
            .disable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(Pull::None)
            .set_function(Pwm::<I>::FUNCTION_V2);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    #[inline]
    pub fn into_i2c<const I: usize>(self) -> Padv2<GLB, N, I2c<I>> {
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(Pull::Up)
            .set_function(I2c::<I>::FUNCTION_V2);
        unsafe {
            self.base.gpio_config[N].write(config);
        }
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as D0 core JTAG.
    #[inline]
    pub fn into_jtag_d0(self) -> Padv2<GLB, N, JtagD0> {
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagD0);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as M0 core JTAG.
    #[inline]
    pub fn into_jtag_m0(self) -> Padv2<GLB, N, JtagM0> {
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagM0);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as LP core JTAG.
    #[inline]
    pub fn into_jtag_lp(self) -> Padv2<GLB, N, JtagLp> {
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagLp);
        unsafe { self.base.gpio_config[N].write(config) };
        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a SPI pin.
    #[inline]
    pub fn into_spi<const I: usize>(self) -> Padv2<GLB, N, Spi<I>> {
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .disable_output()
            .enable_schmitt()
            .set_pull(Pull::Up)
            .set_drive(Drive::Drive0)
            .set_function(Spi::<I>::FUNCTION_V2);
        unsafe {
            self.base.gpio_config[N].write(config);
        }

        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a SDH pin.
    #[inline]
    pub fn into_sdh(self) -> Padv2<GLB, N, Sdh> {
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .disable_output()
            .enable_schmitt()
            .set_pull(Pull::Up)
            .set_drive(Drive::Drive0)
            .set_function(v2::Function::Sdh);
        unsafe {
            self.base.gpio_config[N].write(config);
        }

        Padv2 {
            base: self.base,
            _mode: PhantomData,
        }
    }
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> ErrorType
    for Padv2<GLB, N, Input<M>>
{
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> ErrorType
    for Padv2<GLB, N, Output<M>>
{
    type Error = core::convert::Infallible;
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> InputPin
    for Padv2<GLB, N, Input<M>>
{
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) == 0)
    }
}

impl<GLB: Deref<Target = v2::RegisterBlock>, const N: usize, M> OutputPin
    for Padv2<GLB, N, Output<M>>
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { self.base.gpio_clear[N >> 5].write(1 << (N & 0x1F)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { self.base.gpio_set[N >> 5].write(1 << (N & 0x1F)) };
        Ok(())
    }
}
