use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn dos_mutates_division_operators() {
    let source = r#"
int divide(int lhs, int rhs) {
    return lhs / rhs;
}
"#;

    let mutants = mutants_for_slug(source, "DOS");
    assert!(
        !mutants.is_empty(),
        "expected DOS mutants for division expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["~/", "^/"] {
        assert!(
            replacements.contains(expected),
            "missing DOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
