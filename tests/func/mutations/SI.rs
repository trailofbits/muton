use super::mutants_for_slug;

#[test]
fn si_replaces_store_int_first_argument_with_zero() {
    let source = r#"
() build() {
    var builder = begin_cell();
    store_int(builder, 5, 32);
}
"#;

    let mutants = mutants_for_slug(source, "SI");
    assert!(
        !mutants.is_empty(),
        "expected SI mutants targeting store_int calls"
    );

    assert!(
        mutants.iter().all(|m| m.new_text.trim() == "0"),
        "expected SI mutants to replace the first argument with zero"
    );
}
