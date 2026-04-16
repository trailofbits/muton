use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn sos_mutates_shift_operators() {
    let source = r#"
fun shift(value: int, bits: int): int {
    return value << bits;
}
"#;

    let mutants = mutants_for_slug(source, "SOS");
    assert!(
        !mutants.is_empty(),
        "expected SOS mutants for shift operators"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    assert!(
        replacements.contains(">>"),
        "expected SOS to produce a `>>` replacement; replacements: {replacements:?}"
    );
}
