use crate::func::integration_tests::mutants_for_slug;

#[test]
fn cr_wraps_statements_in_block_comments() {
    let source = r#"
() mutate() {
    var value = 7;
}
"#;

    let mutants = mutants_for_slug(source, "CR");
    assert!(
        !mutants.is_empty(),
        "expected CR mutants to wrap statements in comments"
    );

    for mutant in &mutants {
        let trimmed = mutant.new_text.trim();
        assert!(
            trimmed.starts_with("{-") && trimmed.ends_with("-}"),
            "CR mutant should wrap text in `{{- ... -}}`: {}",
            trimmed
        );
    }
}
