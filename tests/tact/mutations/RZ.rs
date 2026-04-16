use super::first_mutated_source;

#[test]
fn rz_replaces_repeat_condition_with_zero() {
    let source = r#"
    contract C { fun f(a: Int) { repeat (a) { doSomething(); } } }
    "#;

    let mutated = first_mutated_source(source, "RZ").expect("RZ mutant");
    let expected = r#"
    contract C { fun f(a: Int) { repeat (0) { doSomething(); } } }
    "#;
    assert_eq!(mutated, expected);
}
