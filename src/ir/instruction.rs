//! LLVM instruction representation

use super::{Type, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Arithmetic
    Add {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    Sub {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    Mul {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    UDiv {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    SDiv {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    URem {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    SRem {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },

    // Bitwise
    And {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    Or {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    Xor {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    Shl {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    LShr {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },
    AShr {
        result: String,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },

    // Comparison
    ICmp {
        result: String,
        pred: ICmpPredicate,
        ty: Type,
        lhs: Value,
        rhs: Value,
    },

    // Memory
    Load {
        result: String,
        ty: Type,
        ptr: Value,
    },
    Store {
        value: Value,
        ty: Type,
        ptr: Value,
    },
    Alloca {
        result: String,
        ty: Type,
    },

    // Control flow
    Ret {
        value: Option<Value>,
    },
    Br {
        dest: String,
    },
    CondBr {
        cond: Value,
        true_dest: String,
        false_dest: String,
    },
    Call {
        result: Option<String>,
        callee: String,
        args: Vec<Value>,
        ret_ty: Type,
    },
    Phi {
        result: String,
        ty: Type,
        incoming: Vec<(Value, String)>, // (value, block_name)
    },

    // Other
    GetElementPtr {
        result: String,
        ty: Type,
        ptr: Value,
        indices: Vec<Value>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ICmpPredicate {
    Eq,  // Equal
    Ne,  // Not equal
    Slt, // Signed less than
    Sle, // Signed less or equal
    Sgt, // Signed greater than
    Sge, // Signed greater or equal
    Ult, // Unsigned less than
    Ule, // Unsigned less or equal
    Ugt, // Unsigned greater than
    Uge, // Unsigned greater or equal
}

impl Instruction {
    /// Check if this is a terminator instruction
    pub fn is_terminator(&self) -> bool {
        matches!(self, Instruction::Ret { .. } | Instruction::Br { .. } | Instruction::CondBr { .. })
    }

    /// Get the result variable name (if any)
    pub fn result(&self) -> Option<&str> {
        match self {
            Instruction::Add { result, .. }
            | Instruction::Sub { result, .. }
            | Instruction::Mul { result, .. }
            | Instruction::UDiv { result, .. }
            | Instruction::SDiv { result, .. }
            | Instruction::URem { result, .. }
            | Instruction::SRem { result, .. }
            | Instruction::And { result, .. }
            | Instruction::Or { result, .. }
            | Instruction::Xor { result, .. }
            | Instruction::Shl { result, .. }
            | Instruction::LShr { result, .. }
            | Instruction::AShr { result, .. }
            | Instruction::ICmp { result, .. }
            | Instruction::Load { result, .. }
            | Instruction::Alloca { result, .. }
            | Instruction::Phi { result, .. }
            | Instruction::GetElementPtr { result, .. } => Some(result),
            Instruction::Call { result, .. } => result.as_deref(),
            _ => None,
        }
    }
}
