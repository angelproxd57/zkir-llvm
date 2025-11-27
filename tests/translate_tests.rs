//! Translation tests

use zkir_llvm::translate::types::*;
use zkir_llvm::ir::Type;

#[test]
fn test_type_lowering() {
    assert_eq!(lower_type(&Type::Int(1)), Some(ZkType::I32));
    assert_eq!(lower_type(&Type::Int(8)), Some(ZkType::I32));
    assert_eq!(lower_type(&Type::Int(16)), Some(ZkType::I32));
    assert_eq!(lower_type(&Type::Int(32)), Some(ZkType::I32));
    assert_eq!(lower_type(&Type::Int(64)), Some(ZkType::I64));
    assert_eq!(lower_type(&Type::Int(128)), Some(ZkType::I128));
    assert_eq!(lower_type(&Type::Ptr), Some(ZkType::Ptr));
    assert_eq!(lower_type(&Type::Void), None);
}

#[test]
fn test_zk_type_num_regs() {
    assert_eq!(ZkType::I32.num_regs(), 1);
    assert_eq!(ZkType::I64.num_regs(), 2);
    assert_eq!(ZkType::I128.num_regs(), 4);
    assert_eq!(ZkType::Ptr.num_regs(), 1);
}

#[test]
fn test_zk_type_size_bytes() {
    assert_eq!(ZkType::I32.size_bytes(), 4);
    assert_eq!(ZkType::I64.size_bytes(), 8);
    assert_eq!(ZkType::I128.size_bytes(), 16);
    assert_eq!(ZkType::Ptr.size_bytes(), 4);
}

#[test]
fn test_aggregate_type_lowering() {
    // Arrays should lower to pointers (stored in memory)
    let arr = Type::Array(10, Box::new(Type::Int(32)));
    assert_eq!(lower_type(&arr), Some(ZkType::Ptr));

    // Structs should lower to pointers
    let struct_ty = Type::Struct(vec![Type::Int(32), Type::Int(64)]);
    assert_eq!(lower_type(&struct_ty), Some(ZkType::Ptr));
}
