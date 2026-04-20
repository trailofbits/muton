use crate::tolk::integration_tests::first_mutated_source;

#[test]
fn nr_removes_logical_negation() {
    let source = r#"
fun check(flag: bool): bool {
    if (!flag) {
        return false;
    }
    return true;
}
"#;

    let mutated = first_mutated_source(source, "NR").expect("NR mutant");
    assert!(
        mutated.contains("if (flag)"),
        "expected NR mutant to remove `!` from condition; mutated source: {mutated}"
    );
}
