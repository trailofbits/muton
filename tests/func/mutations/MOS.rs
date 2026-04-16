use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn mos_mutates_modulo_operators() {
    let source = r#"
int remainder(int lhs, int rhs) {
    return lhs % rhs;
}
"#;

    let mutants = mutants_for_slug(source, "MOS");
    assert!(
        !mutants.is_empty(),
        "expected MOS mutants for modulo expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["~%", "^%"] {
        assert!(
            replacements.contains(expected),
            "missing MOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
