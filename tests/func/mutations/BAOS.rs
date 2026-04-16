use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn baos_mutates_bitwise_assignment_operators() {
    let source = r#"
() mutate() {
    var flags = 0;
    flags &= 1;
}
"#;

    let mutants = mutants_for_slug(source, "BAOS");
    assert!(
        !mutants.is_empty(),
        "expected BAOS mutants for bitwise assignment expressions"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["|=", "^="] {
        assert!(
            replacements.contains(expected),
            "missing BAOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
