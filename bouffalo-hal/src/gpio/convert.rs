use super::{
    alternate::Alternate,
    input::Input,
    output::Output,
    typestate::{self, Floating, PullDown, PullUp},
};

/// Trait for pad mode conversations.
pub trait IntoPad<GLB, const N: usize> {
    /// Configures the pad to operate as a pull up output pad.
    fn into_pull_up_output(self) -> Output<GLB, N, PullUp>;
    /// Configures the pad to operate as a pull down output pad.
    fn into_pull_down_output(self) -> Output<GLB, N, PullDown>;
    /// Configures the pad to operate as a floating output pad.
    fn into_floating_output(self) -> Output<GLB, N, Floating>;
    /// Configures the pad to operate as a pull up input pad.
    fn into_pull_up_input(self) -> Input<GLB, N, PullUp>;
    /// Configures the pad to operate as a pull down input pad.
    fn into_pull_down_input(self) -> Input<GLB, N, PullDown>;
    /// Configures the pad to operate as a floating input pad.
    fn into_floating_input(self) -> Input<GLB, N, Floating>;
}

/// Trait for GLBv2 pad mode conversations.
pub trait IntoPadv2<GLB, const N: usize> {
    /// Configures the pin to operate as a SPI pin.
    fn into_spi<const I: usize>(self) -> Alternate<GLB, N, typestate::Spi<I>>;
    /// Configures the pin to operate as a SDH pin.
    fn into_sdh(self) -> Alternate<GLB, N, typestate::Sdh>;
    /// Configures the pin to operate as UART signal.
    fn into_uart(self) -> Alternate<GLB, N, typestate::Uart>;
    /// Configures the pin to operate as multi-media cluster UART signal.
    fn into_mm_uart(self) -> Alternate<GLB, N, typestate::MmUart>;
    /// Configures the pin to operate as a pull up Pulse Width Modulation signal pin.
    fn into_pull_up_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>>;
    /// Configures the pin to operate as a pull down Pulse Width Modulation signal pin.
    fn into_pull_down_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>>;
    /// Configures the pin to operate as floating Pulse Width Modulation signal pin.
    fn into_floating_pwm<const I: usize>(self) -> Alternate<GLB, N, typestate::Pwm<I>>;
    /// Configures the pin to operate as an Inter-Integrated Circuit signal pin.
    fn into_i2c<const I: usize>(self) -> Alternate<GLB, N, typestate::I2c<I>>;
    /// Configures the pin to operate as D0 core JTAG.
    fn into_jtag_d0(self) -> Alternate<GLB, N, typestate::JtagD0>;
    /// Configures the pin to operate as M0 core JTAG.
    fn into_jtag_m0(self) -> Alternate<GLB, N, typestate::JtagM0>;
    /// Configures the pin to operate as LP core JTAG.
    fn into_jtag_lp(self) -> Alternate<GLB, N, typestate::JtagLp>;
}
