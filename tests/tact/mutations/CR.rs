use super::first_mutated_source;

#[test]
fn cr_wraps_statement_in_block_comment() {
    let source = r#"
    contract C { fun f() { let x: Int = 1; } }
    "#;

    let mutated = first_mutated_source(source, "CR").expect("CR mutant");
    let expected = r#"
    contract C { fun f() { /* let x: Int = 1; */ } }
    "#;
    assert_eq!(mutated, expected);
}
