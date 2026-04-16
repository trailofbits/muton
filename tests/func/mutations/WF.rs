use super::mutants_for_slug;

#[test]
fn wf_sets_while_condition_to_false() {
    let source = r#"
() loop(int counter) {
    while (counter > 0) {
        counter -= 1;
    }
}
"#;

    let mutants = mutants_for_slug(source, "WF");
    assert!(
        !mutants.is_empty(),
        "expected WF mutants to replace the while condition"
    );

    for mutant in &mutants {
        assert_eq!(mutant.new_text.trim(), "false");
    }
}
