use crate::func::integration_tests::mutants_for_slug;

#[test]
fn r#if_replaces_if_conditions_with_false() {
    let source = r#"
() decide(int value) {
    if (value > 0) {
        return ();
    }
}
"#;

    let mutants = mutants_for_slug(source, "IF");
    assert!(
        !mutants.is_empty(),
        "expected IF mutants to replace the if condition"
    );

    for mutant in &mutants {
        let new_text = mutant.new_text.trim();
        assert!(new_text == "false" || new_text == "(false)");
    }
}
