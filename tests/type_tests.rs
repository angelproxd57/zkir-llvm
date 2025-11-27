//! Type system tests

use zkir_llvm::ir::Type;

#[test]
fn test_type_bit_width() {
    assert_eq!(Type::Int(1).bit_width(), 1);
    assert_eq!(Type::Int(8).bit_width(), 8);
    assert_eq!(Type::Int(16).bit_width(), 16);
    assert_eq!(Type::Int(32).bit_width(), 32);
    assert_eq!(Type::Int(64).bit_width(), 64);
    assert_eq!(Type::Ptr.bit_width(), 32);
    assert_eq!(Type::Void.bit_width(), 0);
}

#[test]
fn test_type_size_in_bytes() {
    assert_eq!(Type::Int(8).size_in_bytes(), 1);
    assert_eq!(Type::Int(16).size_in_bytes(), 2);
    assert_eq!(Type::Int(32).size_in_bytes(), 4);
    assert_eq!(Type::Int(64).size_in_bytes(), 8);
    assert_eq!(Type::Ptr.size_in_bytes(), 4);
}

#[test]
fn test_type_is_scalar() {
    assert!(Type::Int(32).is_scalar());
    assert!(Type::Ptr.is_scalar());
    assert!(!Type::Void.is_scalar());
    assert!(!Type::Array(10, Box::new(Type::Int(32))).is_scalar());
}

#[test]
fn test_type_is_supported() {
    assert!(Type::Int(1).is_supported());
    assert!(Type::Int(8).is_supported());
    assert!(Type::Int(32).is_supported());
    assert!(Type::Int(64).is_supported());
    assert!(Type::Ptr.is_supported());

    // Odd bit widths not supported
    assert!(!Type::Int(7).is_supported());
    assert!(!Type::Int(33).is_supported());
}

#[test]
fn test_array_type() {
    let arr = Type::Array(10, Box::new(Type::Int(32)));
    assert_eq!(arr.size_in_bytes(), 40); // 10 * 4 bytes
    assert!(arr.is_supported());
    assert!(!arr.is_scalar());
    assert!(arr.is_aggregate());
}

#[test]
fn test_struct_type() {
    let fields = vec![Type::Int(32), Type::Int(64), Type::Ptr];
    let struct_ty = Type::Struct(fields);

    assert_eq!(struct_ty.size_in_bytes(), 16); // 4 + 8 + 4
    assert!(struct_ty.is_supported());
    assert!(!struct_ty.is_scalar());
    assert!(struct_ty.is_aggregate());
}
