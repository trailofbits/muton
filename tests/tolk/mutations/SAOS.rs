use std::collections::HashSet;

use crate::tolk::integration_tests::mutants_for_slug;

#[test]
fn saos_mutates_shift_assignment_operators() {
    let source = r#"
fun shift(value: int): int {
    var result = value;
    result <<= 1;
    return result;
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
        "expected SAOS to produce a `>>=` replacement; replacements: {replacements:?}"
    );
}
