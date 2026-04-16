use super::first_mutated_source;

#[test]
fn cr_wraps_statements_in_block_comments() {
    let source = r#"
fun wrap(): int {
    var value = 1;
    return value;
}
"#;

    let mutated = first_mutated_source(source, "CR").expect("CR mutant");
    assert!(
        mutated.contains("/* var value = 1; */"),
        "expected CR mutant to wrap the statement in a block comment; mutated source: {mutated}"
    );
}
