use super::mutants_for_slug;

#[test]
fn it_replaces_if_conditions_with_true() {
    let source = r#"
() decide(int value) {
    if (value > 0) {
        return ();
    }
}
"#;

    let mutants = mutants_for_slug(source, "IT");
    assert!(
        !mutants.is_empty(),
        "expected IT mutants to replace the if condition"
    );

    for mutant in &mutants {
        assert_eq!(mutant.new_text.trim(), "true");
    }
}
