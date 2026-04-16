use super::first_mutated_source;

#[test]
fn if_replaces_condition_with_false() {
    let source = r#"
fun check(a: int, b: int): int {
    if (a > b) {
        return a;
    }
    return b;
}
"#;

    let mutated = first_mutated_source(source, "IF").expect("IF mutant");
    assert!(
        mutated.contains("if (false)"),
        "expected IF mutant to replace the condition with `false`; mutated source: {mutated}"
    );
}
