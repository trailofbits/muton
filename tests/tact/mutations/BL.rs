use crate::tact::integration_tests::mutants_for_slug;

#[test]
fn bl_flips_boolean_literals() {
    let source = r#"
    contract E {
        fun f() {
            if (true) {
                return;
            }
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "BL");
    assert!(
        !mutants.is_empty(),
        "expected BL mutants to flip boolean literals"
    );

    assert!(
        mutants.iter().any(|m| m.new_text.trim() == "false"),
        "expected at least one BL mutant replacing with `false`"
    );
}
