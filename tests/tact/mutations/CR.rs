use crate::tact::integration_tests::first_mutated_source;

#[test]
fn cr_wraps_statement_in_block_comment() {
    let source = r#"
    contract C { fun f() { let x: Int = 1; } }
    "#;

    let mutated = first_mutated_source(source, "CR").expect("CR mutant");
    assert!(
        mutated.contains("/* let x: Int = 1") && mutated.contains("*/"),
        "expected CR mutant to wrap the statement in a block comment; mutated: {mutated}"
    );
}
