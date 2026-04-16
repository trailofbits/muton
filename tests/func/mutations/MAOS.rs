use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn maos_mutates_modulo_assignment_operators() {
    let source = r#"
() adjust(int divisor) {
    var residue = 17;
    residue %= divisor;
}
"#;

    let mutants = mutants_for_slug(source, "MAOS");
    assert!(
        !mutants.is_empty(),
        "expected MAOS mutants for modulo assignment expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["~%=", "^%="] {
        assert!(
            replacements.contains(expected),
            "missing MAOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
