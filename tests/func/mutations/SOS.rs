use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn sos_mutates_shift_operators() {
    let source = r#"
int shift_left(int value, int bits) {
    return value << bits;
}
"#;

    let mutants = mutants_for_slug(source, "SOS");
    assert!(
        !mutants.is_empty(),
        "expected SOS mutants for shift expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in [">>", "~>>", "^>>"] {
        assert!(
            replacements.contains(expected),
            "missing SOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
