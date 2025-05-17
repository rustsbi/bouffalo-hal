use crate::glb::v2::{PwmSignal0, PwmSignal1};

/// Single end signal settings (type state).
pub struct SingleEnd;

/// Differential end signal settings (type state).
pub struct DifferentialEnd;

/// Brushless DC Motor (BLDC) signal settings (type state).
pub struct BrushlessDcMotor;

/// Valid settings for PWM signal 0.
pub trait Signal0 {
    /// Signal value in `PwmConfig` register on `GLB` peripheral.
    const VALUE: PwmSignal0;
}

/// Valid settings for PWM signal 1.
pub trait Signal1 {
    /// Signal value in `PwmConfig` register on `GLB` peripheral.
    const VALUE: PwmSignal1;
}

impl Signal0 for SingleEnd {
    const VALUE: PwmSignal0 = PwmSignal0::SingleEnd;
}

impl Signal0 for DifferentialEnd {
    const VALUE: PwmSignal0 = PwmSignal0::DifferentialEnd;
}

impl Signal1 for SingleEnd {
    const VALUE: PwmSignal1 = PwmSignal1::SingleEnd;
}

impl Signal1 for BrushlessDcMotor {
    const VALUE: PwmSignal1 = PwmSignal1::BrushlessDcMotor;
}

/// Positive signal polarity (type state).
pub struct Positive;

/// Negative signal polarity (type state).
pub struct Negative;

/// Check if target is internally connected to PWM signal, polarity under signal settings.
///
/// It checks if it is connected to PWM group `I`, channel `J` and polarity `P` with signal settings `S`.
#[diagnostic::on_unimplemented(
    message = "this I/O Alternate has no hardware connection to '{P}' polarity signal of PWM group {I}, channel {J} with signal setting {S}"
)]
pub trait HasPwmSignal<S, const I: usize, const J: usize, P> {}

/// Check if target is internally connected to PWM external break signal.
///
/// It checks if it is connected to external break signal of PWM group `I`.
#[diagnostic::on_unimplemented(
    message = "this I/O Alternate has no hardware connection to external break signal of PWM group {I}"
)]
pub trait HasPwmExternalBreak<const I: usize> {}
