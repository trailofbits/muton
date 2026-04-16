use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn aaos_mutates_assignment_arithmetic_operators() {
    let source = r#"
    contract C {
        fun apply() {
            let mut x: Int = 5;
            x += 1;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "AAOS");
    assert!(
        !mutants.is_empty(),
        "expected AAOS mutants for compound assignments"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["-=", "*=", "/="] {
        assert!(
            replacements.contains(expected),
            "missing AAOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
