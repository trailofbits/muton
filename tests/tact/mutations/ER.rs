use crate::tact::integration_tests::first_mutated_source;

#[test]
fn er_replaces_statements_with_require_false() {
    let source = r#"
    contract C { fun f() { let x: Int = 1; } }
    "#;

    let mutated = first_mutated_source(source, "ER").expect("ER mutant");
    assert!(
        mutated.contains("require(false);"),
        "expected ER mutant to insert `require(false);`; mutated: {mutated}"
    );
}
