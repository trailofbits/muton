use super::first_mutated_source;

#[test]
fn if_replaces_condition_with_false() {
    let source = r#"
    contract C { fun f(a: Int, b: Int) { if (a > b) { } } }
    "#;

    let mutated = first_mutated_source(source, "IF").expect("IF mutant");
    let expected = r#"
    contract C { fun f(a: Int, b: Int) { if (false) { } } }
    "#;
    assert_eq!(mutated, expected);
}
