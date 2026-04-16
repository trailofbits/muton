use super::mutants_for_slug;

#[test]
fn sc_replaces_store_coins_first_argument_with_zero() {
    let source = r#"
() build() {
    var builder = begin_cell();
    store_coins(builder, 5);
}
"#;

    let mutants = mutants_for_slug(source, "SC");
    assert!(
        !mutants.is_empty(),
        "expected SC mutants targeting store_coins calls"
    );

    assert!(
        mutants.iter().all(|m| m.new_text.trim() == "0"),
        "expected SC mutants to replace the first argument with zero"
    );
}
