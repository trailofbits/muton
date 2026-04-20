use crate::tact::integration_tests::mutants_for_slug;

#[test]
fn tf_replaces_ternary_conditions_with_false() {
    let source = r#"
    contract F {
        fun f(a: Int): Int {
            return a > 0 ? 1 : 2;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "TF");
    assert!(
        !mutants.is_empty(),
        "expected TF mutants for ternary expressions"
    );

    assert!(
        mutants.iter().any(|m| m.new_text.trim() == "false"),
        "expected TF mutant replacing condition with `false`"
    );
}
