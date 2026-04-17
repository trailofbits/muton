use crate::tact::integration_tests::first_mutated_source;

#[test]
fn uf_replaces_do_until_condition_with_false() {
    let source = r#"
    contract C { fun f(a: Int) { do { let x: Int = a; } until (a == 0); } }
    "#;

    let mutated = first_mutated_source(source, "UF").expect("UF mutant");
    let expected = r#"
    contract C { fun f(a: Int) { do { let x: Int = a; } until (false); } }
    "#;
    assert_eq!(mutated, expected);
}
