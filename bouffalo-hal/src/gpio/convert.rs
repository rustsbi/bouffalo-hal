use super::{
    input::Input,
    output::Output,
    typestate::{Floating, PullDown, PullUp},
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
