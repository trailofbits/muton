use std::collections::HashSet;

use crate::func::integration_tests::mutants_for_slug;

#[test]
fn aaos_mutates_arithmetic_assignment_operators() {
    let source = r#"
() mutate() {
    var total = 10;
    total += 5;
}
"#;

    let mutants = mutants_for_slug(source, "AAOS");
    assert!(
        !mutants.is_empty(),
        "expected AAOS mutants for arithmetic assignment expressions"
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
