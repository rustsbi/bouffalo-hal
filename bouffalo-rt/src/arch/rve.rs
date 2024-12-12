//! RISC-V RV32E and RV64E structures.

/// RISC-V program stack.
///
/// RV32E stacks are 4-byte aligned.
#[repr(align(4))]
pub struct Stack<const N: usize>(pub(crate) [u8; N]);

/// RISC-V 'E' instruction base Trap stack frame declaration.
#[repr(C)]
pub struct TrapFrame {
    /// Return address register.
    pub ra: usize,
    /// Temporary register 0.
    pub t0: usize,
    /// Temporary register 1.
    pub t1: usize,
    /// Temporary register 2.
    pub t2: usize,
    /// Argument register 0.
    pub a0: usize,
    /// Argument register 1.
    pub a1: usize,
    /// Argument register 2.
    pub a2: usize,
    /// Argument register 3.
    pub a3: usize,
    /// Argument register 4.
    pub a4: usize,
    /// Argument register 5.
    pub a5: usize,
    /// Machine cause register.
    pub mcause: usize,
    /// Machine exception program counter register.
    pub mepc: usize,
    /// Machine status register.
    pub mstatus: usize,
}
