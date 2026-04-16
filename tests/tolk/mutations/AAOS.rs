use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn aaos_mutates_arithmetic_assignment_operators() {
    let source = r#"
fun adjust(): int {
    var value = 10;
    value += 1;
    return value;
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
