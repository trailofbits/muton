use std::collections::HashSet;

use crate::tact::integration_tests::mutants_for_slug;

#[test]
fn saos_mutates_shift_assignment_operators() {
    let source = r#"
    contract M {
        fun apply(a: Int) {
            let x: Int = a;
            x <<= 1;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "SAOS");
    assert!(
        !mutants.is_empty(),
        "expected SAOS mutants for shift assignments"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    assert!(
        replacements.contains(">>="),
        "missing SAOS replacement `>>=`; replacements: {replacements:?}"
    );
}
