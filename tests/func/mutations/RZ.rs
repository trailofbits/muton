use super::mutants_for_slug;

#[test]
fn rz_sets_repeat_count_to_zero() {
    let source = r#"
() spin() {
    repeat(5) {
        do_nothing();
    }
}
"#;

    let mutants = mutants_for_slug(source, "RZ");
    assert!(
        !mutants.is_empty(),
        "expected RZ mutants to adjust repeat counts"
    );

    for mutant in &mutants {
        assert_eq!(mutant.new_text.trim(), "0");
    }
}
