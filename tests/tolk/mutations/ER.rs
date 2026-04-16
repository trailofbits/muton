use super::first_mutated_source;

#[test]
fn er_replaces_statements_with_throw() {
    let source = r#"
fun fail(): int {
    var value = 1;
    return value;
}
"#;

    let mutated = first_mutated_source(source, "ER").expect("ER mutant");
    assert!(
        mutated.contains("throw 65535;"),
        "expected ER mutant to replace the statement with `throw 65535;`; mutated source: {mutated}"
    );
}
