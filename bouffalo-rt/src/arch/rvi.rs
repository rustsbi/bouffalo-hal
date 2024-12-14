//! RISC-V RV32I and RV64I structures.

/// RISC-V program stack.
///
/// In standard RISC-V ABI specification, the stack grows downward and
/// the stack pointer is always kept 16-byte aligned.
#[repr(align(16))]
pub struct Stack<const N: usize>(pub(crate) [u8; N]);

/// RISC-V 'I' instruction base Trap stack frame declaration.
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
    /// Argument register 6.
    pub a6: usize,
    /// Argument register 7.
    pub a7: usize,
    /// Temporary register 3.
    pub t3: usize,
    /// Temporary register 4.
    pub t4: usize,
    /// Temporary register 5.
    pub t5: usize,
    /// Temporary register 6.
    pub t6: usize,
    /// Machine cause register.
    pub mcause: usize,
    /// Machine exception program counter register.
    pub mepc: usize,
    /// Machine status register.
    pub mstatus: usize,
}
