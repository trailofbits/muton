use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn cos_mutates_comparison_operators() {
    let source = r#"
() classify(int lhs, int rhs) {
    if (lhs == rhs) {
        return ();
    }
}
"#;

    let mutants = mutants_for_slug(source, "COS");
    assert!(
        !mutants.is_empty(),
        "expected COS mutants for comparison expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["!=", "<", "<=", ">", ">="] {
        assert!(
            replacements.contains(expected),
            "missing COS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
