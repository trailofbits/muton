use crate::func::integration_tests::mutants_for_slug;

#[test]
fn inf_replaces_ifnot_conditions_with_false() {
    let source = r#"
() decide(int value) {
    ifnot (
        value == 0
    ) {
        return ();
    }
}
"#;

    let mutants = mutants_for_slug(source, "INF");
    assert!(
        !mutants.is_empty(),
        "expected INF mutants to replace the ifnot condition"
    );

    for mutant in &mutants {
        let new_text = mutant.new_text.trim();
        assert!(new_text == "false" || new_text == "(false)");
    }
}
