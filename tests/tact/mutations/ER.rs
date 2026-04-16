use super::first_mutated_source;

#[test]
fn er_replaces_statements_with_require_false() {
    let source = r#"
    contract C { fun f() { let x: Int = 1; } }
    "#;

    let mutated = first_mutated_source(source, "ER").expect("ER mutant");
    let expected = r#"
    contract C { fun f() { require(false); } }
    "#;
    assert_eq!(mutated, expected);
}
