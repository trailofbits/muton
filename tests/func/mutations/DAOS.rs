use std::collections::HashSet;

use crate::func::integration_tests::mutants_for_slug;

#[test]
fn daos_mutates_division_assignment_operators() {
    let source = r#"
() update(int denom) {
    var value = 64;
    value /= denom;
}
"#;

    let mutants = mutants_for_slug(source, "DAOS");
    assert!(
        !mutants.is_empty(),
        "expected DAOS mutants for division assignment expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["~/=", "^/="] {
        assert!(
            replacements.contains(expected),
            "missing DAOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
