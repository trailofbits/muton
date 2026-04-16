use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn saos_mutates_shift_assignment_operators() {
    let source = r#"
() shift(int bits) {
    var value = 1;
    value <<= bits;
}
"#;

    let mutants = mutants_for_slug(source, "SAOS");
    assert!(
        !mutants.is_empty(),
        "expected SAOS mutants for shift assignment expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in [">>=", "~>>=", "^>>="] {
        assert!(
            replacements.contains(expected),
            "missing SAOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
