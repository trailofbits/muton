use crate::tolk::integration_tests::first_mutated_source;

#[test]
fn wf_replaces_while_condition_with_false() {
    let source = r#"
fun countdown(limit: int): int {
    var current = limit;
    while (current > 0) {
        current = current - 1;
    }
    return current;
}
"#;

    let mutated = first_mutated_source(source, "WF").expect("WF mutant");
    assert!(
        mutated.contains("while (false)"),
        "expected WF mutant to replace the while condition with `false`; mutated source: {mutated}"
    );
}
