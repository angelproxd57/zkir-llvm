//! Instruction tests

use zkir_llvm::ir::{instruction::*, Type, Value};

#[test]
fn test_instruction_is_terminator() {
    let ret = Instruction::Ret {
        value: Some(Value::const_i32(0)),
    };
    assert!(ret.is_terminator());

    let br = Instruction::Br {
        dest: "label".to_string(),
    };
    assert!(br.is_terminator());

    let cond_br = Instruction::CondBr {
        cond: Value::ConstBool(true),
        true_dest: "true_label".to_string(),
        false_dest: "false_label".to_string(),
    };
    assert!(cond_br.is_terminator());

    let add = Instruction::Add {
        result: "x".to_string(),
        ty: Type::Int(32),
        lhs: Value::const_i32(1),
        rhs: Value::const_i32(2),
    };
    assert!(!add.is_terminator());
}

#[test]
fn test_instruction_result() {
    let add = Instruction::Add {
        result: "sum".to_string(),
        ty: Type::Int(32),
        lhs: Value::const_i32(1),
        rhs: Value::const_i32(2),
    };
    assert_eq!(add.result(), Some("sum"));

    let ret = Instruction::Ret {
        value: Some(Value::const_i32(0)),
    };
    assert_eq!(ret.result(), None);

    let store = Instruction::Store {
        value: Value::const_i32(42),
        ty: Type::Int(32),
        ptr: Value::local("ptr"),
    };
    assert_eq!(store.result(), None);
}

#[test]
fn test_arithmetic_instructions() {
    let instructions = vec![
        Instruction::Add {
            result: "r1".to_string(),
            ty: Type::Int(32),
            lhs: Value::local("a"),
            rhs: Value::local("b"),
        },
        Instruction::Sub {
            result: "r2".to_string(),
            ty: Type::Int(32),
            lhs: Value::local("a"),
            rhs: Value::local("b"),
        },
        Instruction::Mul {
            result: "r3".to_string(),
            ty: Type::Int(32),
            lhs: Value::local("a"),
            rhs: Value::local("b"),
        },
    ];

    assert_eq!(instructions[0].result(), Some("r1"));
    assert_eq!(instructions[1].result(), Some("r2"));
    assert_eq!(instructions[2].result(), Some("r3"));
}

#[test]
fn test_icmp_predicates() {
    use ICmpPredicate::*;

    let predicates = vec![Eq, Ne, Slt, Sle, Sgt, Sge, Ult, Ule, Ugt, Uge];
    assert_eq!(predicates.len(), 10);
}

#[test]
fn test_memory_instructions() {
    let load = Instruction::Load {
        result: "val".to_string(),
        ty: Type::Int(32),
        ptr: Value::local("ptr"),
    };
    assert_eq!(load.result(), Some("val"));

    let store = Instruction::Store {
        value: Value::const_i32(42),
        ty: Type::Int(32),
        ptr: Value::local("ptr"),
    };
    assert_eq!(store.result(), None);

    let alloca = Instruction::Alloca {
        result: "local_ptr".to_string(),
        ty: Type::Int(32),
    };
    assert_eq!(alloca.result(), Some("local_ptr"));
}
