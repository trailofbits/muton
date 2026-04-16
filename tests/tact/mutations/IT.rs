use super::first_mutated_source;

#[test]
fn it_replaces_condition_with_true() {
    let source = r#"
    contract C { fun f(a: Int, b: Int) { if (a > b) { } } }
    "#;

    let mutated = first_mutated_source(source, "IT").expect("IT mutant");
    let expected = r#"
    contract C { fun f(a: Int, b: Int) { if (true) { } } }
    "#;
    assert_eq!(mutated, expected);
}
