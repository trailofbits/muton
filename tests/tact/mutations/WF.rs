use super::first_mutated_source;

#[test]
fn wf_replaces_while_condition_with_false() {
    let source = r#"
    contract C { fun f(a: Int) { while (a > 0) { } } }
    "#;

    let mutated = first_mutated_source(source, "WF").expect("WF mutant");
    let expected = r#"
    contract C { fun f(a: Int) { while (false) { } } }
    "#;
    assert_eq!(mutated, expected);
}
