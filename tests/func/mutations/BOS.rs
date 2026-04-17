use std::collections::HashSet;

use crate::func::integration_tests::mutants_for_slug;

#[test]
fn bos_mutates_bitwise_operators() {
    let source = r#"
int mask(int a, int b) {
    return a & b;
}
"#;

    let mutants = mutants_for_slug(source, "BOS");
    assert!(
        !mutants.is_empty(),
        "expected BOS mutants for bitwise expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["|", "^"] {
        assert!(
            replacements.contains(expected),
            "missing BOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
