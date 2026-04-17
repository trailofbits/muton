use std::collections::HashSet;

use crate::tolk::integration_tests::mutants_for_slug;

#[test]
fn aos_mutates_arithmetic_operators() {
    let source = r#"
fun combine(a: int, b: int): int {
    return a + b;
}
"#;

    let mutants = mutants_for_slug(source, "AOS");
    assert!(
        !mutants.is_empty(),
        "expected AOS mutants for arithmetic expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["-", "*", "/"] {
        assert!(
            replacements.contains(expected),
            "missing AOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
