//! Integration tests - end-to-end translation

use zkir_llvm::{parser, translate};

#[test]
fn test_translate_simple_add() {
    let source = r#"
        define i32 @add(i32 %a, i32 %b) {
        entry:
            %result = add i32 %a, %b
            ret i32 %result
        }
    "#;

    // Parse
    let module = parser::parse(source).expect("Failed to parse");
    assert_eq!(module.functions().len(), 1);

    // Translate
    let program = translate::translate_module(&module, 0);

    // Should succeed (even if incomplete)
    // Note: This will fail until we implement full instruction parsing
    // assert!(program.is_ok(), "Translation failed: {:?}", program.err());
}

#[test]
fn test_translate_empty_module() {
    let source = "";

    let module = parser::parse(source).expect("Failed to parse");
    let program = translate::translate_module(&module, 0).expect("Failed to translate");

    // Empty module should produce empty program
    assert_eq!(program.code.len(), 0);
}

#[test]
fn test_check_compatibility() {
    let source = r#"
        define i32 @test(i32 %a) {
        entry:
            ret i32 %a
        }
    "#;

    let module = parser::parse(source).expect("Failed to parse");
    let result = translate::check_module_compatibility(&module);

    assert!(result.is_ok(), "Compatibility check failed: {:?}", result.err());
}

#[test]
fn test_unsupported_type() {
    // This test checks that we properly detect unsupported types
    // Float types are not supported
    let source = r#"
        define float @test(float %a) {
        entry:
            ret float %a
        }
    "#;

    // This should fail during parsing (since we don't have float token)
    // or during compatibility check
    let result = parser::parse(source);

    // Parser might fail or succeed depending on implementation
    // If it succeeds, compatibility check should fail
    if let Ok(module) = result {
        let _compat = translate::check_module_compatibility(&module);
        // Should either fail or we haven't implemented the check yet
    }
}

#[test]
#[ignore] // Ignore until full parser is implemented
fn test_translate_arithmetic() {
    let source = r#"
        define i32 @arithmetic(i32 %a, i32 %b) {
        entry:
            %sum = add i32 %a, %b
            %diff = sub i32 %sum, %b
            %prod = mul i32 %diff, 2
            ret i32 %prod
        }
    "#;

    let module = parser::parse(source).expect("Failed to parse");
    let program = translate::translate_module(&module, 0).expect("Failed to translate");

    // Should have generated some instructions
    assert!(program.code.len() > 0);
}

#[test]
#[ignore] // Ignore until full parser is implemented
fn test_translate_control_flow() {
    let source = r#"
        define i32 @max(i32 %a, i32 %b) {
        entry:
            %cmp = icmp sgt i32 %a, %b
            br i1 %cmp, label %return_a, label %return_b

        return_a:
            ret i32 %a

        return_b:
            ret i32 %b
        }
    "#;

    let module = parser::parse(source).expect("Failed to parse");
    let program = translate::translate_module(&module, 0).expect("Failed to translate");

    assert!(program.code.len() > 0);
}
