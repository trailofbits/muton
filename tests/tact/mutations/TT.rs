use super::mutants_for_slug;

#[test]
fn tt_replaces_ternary_conditions_with_true() {
    let source = r#"
    contract F {
        fun f(a: Int): Int {
            return if (a > 0) 1 else 2;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "TT");
    assert!(
        !mutants.is_empty(),
        "expected TT mutants for ternary expressions"
    );

    assert!(
        mutants.iter().any(|m| m.new_text.trim() == "true"),
        "expected TT mutant replacing condition with `true`"
    );
}
