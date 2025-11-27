//! Translation context

use super::{TranslateError, TranslateResult};
use crate::ir::{Type, Value};
use std::collections::HashMap;
use zkir_spec::{Instruction, Register};

/// Location where a value is stored
#[derive(Debug, Clone)]
pub enum Location {
    /// Single register
    Reg(Register),

    /// Register pair (for 64-bit values)
    RegPair { lo: Register, hi: Register },

    /// Register quad (for 128-bit values)
    RegQuad {
        r0: Register,
        r1: Register,
        r2: Register,
        r3: Register,
    },

    /// Stack slot (offset from frame pointer)
    Stack(i32),

    /// Constant value
    Const(i64),
}

/// Translation context for a single function
pub struct TranslationContext {
    /// Function name
    function_name: String,

    /// Value bindings (LLVM value name -> location)
    bindings: HashMap<String, Location>,

    /// Generated instructions
    instructions: Vec<Instruction>,

    /// Label positions (block name -> instruction index)
    labels: HashMap<String, u32>,

    /// Pending fixups (instruction index, label name)
    fixups: Vec<(usize, String)>,

    /// Next available temporary register
    next_temp: u8,

    /// Stack frame size
    stack_size: i32,

    /// Current block name
    current_block: String,
}

impl TranslationContext {
    pub fn new(function_name: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            bindings: HashMap::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
            fixups: Vec::new(),
            next_temp: 8, // Start at t0 (r8)
            stack_size: 0,
            current_block: String::new(),
        }
    }

    /// Start a new basic block
    pub fn start_block(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.current_block = name.clone();
        self.labels
            .insert(name, self.instructions.len() as u32);
    }

    /// Emit an instruction
    pub fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    /// Allocate a temporary register
    pub fn alloc_temp(&mut self) -> TranslateResult<Register> {
        // Available temps: t0-t7 (r8-r15), t8-t11 (r24-r27)
        let reg = if self.next_temp < 16 {
            Register::from_index(self.next_temp as usize).unwrap()
        } else if self.next_temp < 20 {
            Register::from_index((self.next_temp + 8) as usize).unwrap() // Skip s0-s7
        } else {
            return Err(TranslateError::OutOfRegisters);
        };

        self.next_temp += 1;
        Ok(reg)
    }

    /// Allocate a register pair for 64-bit value
    pub fn alloc_reg_pair(&mut self) -> TranslateResult<(Register, Register)> {
        let lo = self.alloc_temp()?;
        let hi = self.alloc_temp()?;
        Ok((lo, hi))
    }

    /// Allocate stack space
    pub fn alloc_stack(&mut self, size: i32) -> i32 {
        self.stack_size += size;
        -self.stack_size
    }

    /// Bind a value to a location
    pub fn bind(&mut self, name: impl Into<String>, loc: Location) {
        self.bindings.insert(name.into(), loc);
    }

    /// Bind a parameter
    pub fn bind_parameter(
        &mut self,
        name: &str,
        ty: &Type,
        index: usize,
    ) -> TranslateResult<()> {
        let loc = if index < 4 {
            // First 4 parameters in a0-a3 (r4-r7)
            Location::Reg(Register::from_index(4 + index).unwrap())
        } else {
            // Rest on stack
            Location::Stack((index - 4) as i32 * 4)
        };

        self.bind(name, loc);
        Ok(())
    }

    /// Get the location of a value
    pub fn get_location(&self, name: &str) -> TranslateResult<&Location> {
        self.bindings
            .get(name)
            .ok_or_else(|| TranslateError::UndefinedValue(name.to_string()))
    }

    /// Load a value into a register
    pub fn load_value(&mut self, value: &Value) -> TranslateResult<Register> {
        match value {
            Value::Local(name) => {
                let loc = self.get_location(name)?.clone();
                match loc {
                    Location::Reg(r) => Ok(r),
                    Location::Stack(offset) => {
                        let rd = self.alloc_temp()?;
                        self.emit(Instruction::Lw {
                            rd,
                            rs1: Register::FP,
                            imm: offset as i16,
                        });
                        Ok(rd)
                    }
                    Location::Const(c) => {
                        let rd = self.alloc_temp()?;
                        self.emit_load_imm(rd, c as u32);
                        Ok(rd)
                    }
                    _ => Err(TranslateError::UnsupportedType(Type::Int(64))),
                }
            }
            Value::ConstInt { value, .. } => {
                let rd = self.alloc_temp()?;
                self.emit_load_imm(rd, *value as u32);
                Ok(rd)
            }
            Value::ConstBool(b) => {
                let rd = self.alloc_temp()?;
                self.emit_load_imm(rd, if *b { 1 } else { 0 });
                Ok(rd)
            }
            _ => Err(TranslateError::UnsupportedInstruction(format!(
                "load {:?}",
                value
            ))),
        }
    }

    /// Load a 64-bit value as a register pair
    pub fn load_value_pair(&mut self, value: &Value) -> TranslateResult<(Register, Register)> {
        match value {
            Value::Local(name) => {
                let loc = self.get_location(name)?.clone();
                match loc {
                    Location::RegPair { lo, hi } => Ok((lo, hi)),
                    _ => Err(TranslateError::UnsupportedType(Type::Int(64))),
                }
            }
            Value::ConstInt { value, .. } => {
                let lo = self.alloc_temp()?;
                let hi = self.alloc_temp()?;
                self.emit_load_imm(lo, (*value & 0xFFFFFFFF) as u32);
                self.emit_load_imm(hi, ((*value >> 32) & 0xFFFFFFFF) as u32);
                Ok((lo, hi))
            }
            _ => Err(TranslateError::UnsupportedInstruction(format!(
                "load pair {:?}",
                value
            ))),
        }
    }

    /// Emit instructions to load a 32-bit immediate
    pub fn emit_load_imm(&mut self, rd: Register, value: u32) {
        if value == 0 {
            // Move zero
            self.emit(Instruction::Add {
                rd,
                rs1: Register::ZERO,
                rs2: Register::ZERO,
            });
        } else if value < 2048 {
            // Small positive immediate
            self.emit(Instruction::Addi {
                rd,
                rs1: Register::ZERO,
                imm: value as i16,
            });
        } else {
            // Full 32-bit: LUI + ADDI
            let upper = ((value + 0x800) >> 12) & 0xFFFFF;
            let lower = (value as i32) & 0xFFF;

            self.emit(Instruction::Lui {
                rd,
                imm: upper as i32,
            });

            if lower != 0 {
                self.emit(Instruction::Addi {
                    rd,
                    rs1: rd,
                    imm: lower as i16,
                });
            }
        }
    }

    /// Record a label fixup for later resolution
    pub fn add_fixup(&mut self, label: impl Into<String>) {
        let index = self.instructions.len() - 1; // Last emitted instruction
        self.fixups.push((index, label.into()));
    }

    /// Resolve all label fixups
    pub fn resolve_labels(&mut self) -> TranslateResult<()> {
        for (index, label) in &self.fixups {
            let target = self
                .labels
                .get(label)
                .ok_or_else(|| TranslateError::InvalidBranch(label.clone()))?;

            let offset = (*target as i32 - *index as i32) * 4;

            // Update the instruction with the resolved offset
            // This is a simplified approach - in reality you'd need to
            // properly patch the instruction based on its type
            // TODO: Implement proper instruction patching
        }

        Ok(())
    }

    /// Convert context into instructions
    pub fn into_instructions(self) -> Vec<Instruction> {
        self.instructions
    }

    /// Get the function name
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    pub fn current_index(&self) -> u32 {
        self.instructions.len() as u32
    }
}
