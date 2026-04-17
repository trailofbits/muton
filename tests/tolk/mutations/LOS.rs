use std::collections::HashSet;

use crate::tolk::integration_tests::mutants_for_slug;

#[test]
fn los_mutates_logical_operators() {
    let source = r#"
fun should_run(a: bool, b: bool): bool {
    return a && b;
}
"#;

    let mutants = mutants_for_slug(source, "LOS");
    assert!(
        !mutants.is_empty(),
        "expected LOS mutants for logical operators"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    assert!(
        replacements.contains("||"),
        "expected LOS to include logical OR replacement; replacements: {replacements:?}"
    );
}
