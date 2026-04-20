use crate::func::integration_tests::mutants_for_slug;

#[test]
fn los_mutates_logical_operators() {
    let source = r#"
int both(int lhs, int rhs) {
    return lhs && rhs;
}
"#;

    let mutants = mutants_for_slug(source, "LOS");
    assert!(
        !mutants.is_empty(),
        "expected LOS mutants for logical expressions"
    );

    assert!(
        mutants.iter().any(|m| m.new_text.contains("||")),
        "expected LOS mutants to include `||` replacement"
    );
}
