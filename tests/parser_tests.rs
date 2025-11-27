//! Parser unit tests

use zkir_llvm::parser;
use zkir_llvm::ir::Type;

#[test]
fn test_parse_simple_function() {
    let source = r#"
        define i32 @add(i32 %a, i32 %b) {
        entry:
            %result = add i32 %a, %b
            ret i32 %result
        }
    "#;

    let result = parser::parse(source);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let module = result.unwrap();
    assert_eq!(module.functions().len(), 1);

    let func = &module.functions()[0];
    assert_eq!(func.name(), "add");
    assert_eq!(func.ret_ty(), &Type::Int(32));
    assert_eq!(func.params().len(), 2);
}

#[test]
fn test_parse_void_function() {
    let source = r#"
        define void @noop() {
        entry:
            ret void
        }
    "#;

    let result = parser::parse(source);
    assert!(result.is_ok());

    let module = result.unwrap();
    let func = &module.functions()[0];
    assert_eq!(func.ret_ty(), &Type::Void);
}

#[test]
fn test_parse_multiple_functions() {
    let source = r#"
        define i32 @add(i32 %a, i32 %b) {
        entry:
            %result = add i32 %a, %b
            ret i32 %result
        }

        define i32 @sub(i32 %a, i32 %b) {
        entry:
            %result = sub i32 %a, %b
            ret i32 %result
        }
    "#;

    let result = parser::parse(source);
    assert!(result.is_ok());

    let module = result.unwrap();
    assert_eq!(module.functions().len(), 2);
    assert_eq!(module.functions()[0].name(), "add");
    assert_eq!(module.functions()[1].name(), "sub");
}

#[test]
fn test_parse_types() {
    let source = r#"
        define i64 @test(i1 %a, i8 %b, i16 %c, i32 %d, i64 %e, ptr %p) {
        entry:
            ret i64 0
        }
    "#;

    let result = parser::parse(source);
    assert!(result.is_ok());

    let module = result.unwrap();
    let func = &module.functions()[0];

    assert_eq!(func.params().len(), 6);
    assert_eq!(func.params()[0].1, Type::Int(1));
    assert_eq!(func.params()[1].1, Type::Int(8));
    assert_eq!(func.params()[2].1, Type::Int(16));
    assert_eq!(func.params()[3].1, Type::Int(32));
    assert_eq!(func.params()[4].1, Type::Int(64));
    assert_eq!(func.params()[5].1, Type::Ptr);
}

#[test]
fn test_parse_empty_module() {
    let source = "";
    let result = parser::parse(source);
    assert!(result.is_ok());

    let module = result.unwrap();
    assert_eq!(module.functions().len(), 0);
}

#[test]
fn test_parse_with_comments() {
    let source = r#"
        ; This is a comment
        define i32 @add(i32 %a, i32 %b) { ; inline comment
        entry:
            ; Another comment
            %result = add i32 %a, %b
            ret i32 %result
        }
    "#;

    let result = parser::parse(source);
    assert!(result.is_ok());
}
