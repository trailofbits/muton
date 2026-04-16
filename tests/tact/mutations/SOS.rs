use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn sos_shuffles_shift_operators() {
    let source = r#"
    contract M {
        fun f(a: Int, b: Int): Int {
            return a << b;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "SOS");
    assert!(
        !mutants.is_empty(),
        "expected SOS mutants to shuffle shift operators"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    assert!(
        replacements.contains(">>"),
        "missing SOS replacement `>>`; replacements: {replacements:?}"
    );
}
