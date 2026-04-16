use super::mutants_for_slug;

#[test]
fn uf_sets_until_condition_to_false() {
    let source = r#"
() loop_until(int flag) {
    do {
        flag -= 1;
    } until (flag == 0);
}
"#;

    let mutants = mutants_for_slug(source, "UF");
    assert!(
        !mutants.is_empty(),
        "expected UF mutants to replace the until condition"
    );

    for mutant in &mutants {
        assert_eq!(mutant.new_text.trim(), "false");
    }
}
