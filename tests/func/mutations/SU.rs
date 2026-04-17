use crate::func::integration_tests::mutants_for_slug;

#[test]
fn su_replaces_store_uint_first_argument_with_zero() {
    let source = r#"
() build() {
    var builder = begin_cell();
    store_uint(builder, 5, 32);
}
"#;

    let mutants = mutants_for_slug(source, "SU");
    assert!(
        !mutants.is_empty(),
        "expected SU mutants targeting store_uint calls"
    );

    assert!(
        mutants.iter().all(|m| m.new_text.trim() == "0"),
        "expected SU mutants to replace the first argument with zero"
    );
}
