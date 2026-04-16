use super::mutants_for_slug;

#[test]
fn int_replaces_ifnot_conditions_with_true() {
    let source = r#"
() decide(int value) {
    ifnot (
        value == 0
    ) {
        return ();
    }
}
"#;

    let mutants = mutants_for_slug(source, "INT");
    assert!(
        !mutants.is_empty(),
        "expected INT mutants to replace the ifnot condition"
    );

    for mutant in &mutants {
        assert_eq!(mutant.new_text.trim(), "true");
    }
}
