//! Value tests

use zkir_llvm::ir::{Type, Value};

#[test]
fn test_const_i32() {
    let v = Value::const_i32(42);
    match v {
        Value::ConstInt { value, ty } => {
            assert_eq!(value, 42);
            assert_eq!(ty, Type::Int(32));
        }
        _ => panic!("Expected ConstInt"),
    }
}

#[test]
fn test_const_i64() {
    let v = Value::const_i64(0x1234567890ABCDEF);
    match v {
        Value::ConstInt { value, ty } => {
            assert_eq!(value, 0x1234567890ABCDEF);
            assert_eq!(ty, Type::Int(64));
        }
        _ => panic!("Expected ConstInt"),
    }
}

#[test]
fn test_const_bool() {
    let v_true = Value::ConstBool(true);
    let v_false = Value::ConstBool(false);

    assert_eq!(v_true, Value::ConstBool(true));
    assert_eq!(v_false, Value::ConstBool(false));
}

#[test]
fn test_local_value() {
    let v = Value::local("my_var");
    match v {
        Value::Local(name) => {
            assert_eq!(name, "my_var");
        }
        _ => panic!("Expected Local"),
    }
}

#[test]
fn test_null_value() {
    let v = Value::Null;
    assert_eq!(v, Value::Null);
}

#[test]
fn test_undef_value() {
    let v = Value::Undef;
    assert_eq!(v, Value::Undef);
}
