//! Register allocation

pub mod linear;

pub use linear::LinearScan;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegAllocError {
    #[error("Out of registers")]
    OutOfRegisters,

    #[error("Invalid register: {0}")]
    InvalidRegister(u8),

    #[error("Spill failed: {0}")]
    SpillFailed(String),
}

pub type RegAllocResult<T> = Result<T, RegAllocError>;

/// Virtual register identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualReg(pub u32);

/// Physical register identifier
pub type PhysicalReg = zkir_spec::Register;

/// Live interval for a virtual register
#[derive(Debug, Clone)]
pub struct LiveInterval {
    pub vreg: VirtualReg,
    pub start: u32,
    pub end: u32,
}

impl LiveInterval {
    pub fn new(vreg: VirtualReg, start: u32, end: u32) -> Self {
        Self { vreg, start, end }
    }

    pub fn overlaps(&self, other: &LiveInterval) -> bool {
        self.start < other.end && other.start < self.end
    }
}
