//! Bytecode emission

use anyhow::Result;
use zkir_spec::Program;

/// Emit ZK IR program to bytecode
pub fn emit_program(program: &Program) -> Result<Vec<u8>> {
    // Use bincode for serialization
    bincode::serialize(program).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_empty_program() {
        let program = Program::new(Vec::new());
        let bytes = emit_program(&program).unwrap();
        assert!(!bytes.is_empty());
    }
}
