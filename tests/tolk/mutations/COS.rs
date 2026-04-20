use std::collections::HashSet;

use crate::tolk::integration_tests::mutants_for_slug;

#[test]
fn cos_shuffles_comparison_operators() {
    let source = r#"
fun compare(a: int, b: int): int {
    if (a == b) {
        return 0;
    }
    return 1;
}
"#;

    let mutants = mutants_for_slug(source, "COS");
    assert!(
        !mutants.is_empty(),
        "expected COS mutants to shuffle comparison operators"
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
