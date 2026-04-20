use crate::tolk::integration_tests::first_mutated_source;

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
        mutated.contains("/* var value = 1;") || mutated.contains("/* return value"),
        "expected CR mutant to wrap a statement in a block comment; mutated source: {mutated}"
    );
}
