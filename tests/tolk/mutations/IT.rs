use crate::tolk::integration_tests::first_mutated_source;

#[test]
fn it_replaces_condition_with_true() {
    let source = r#"
fun check(a: int, b: int): int {
    if (a > b) {
        return a;
    }
    return b;
}
"#;

    let mutated = first_mutated_source(source, "IT").expect("IT mutant");
    assert!(
        mutated.contains("if (true)"),
        "expected IT mutant to replace the condition with `true`; mutated source: {mutated}"
    );
}
