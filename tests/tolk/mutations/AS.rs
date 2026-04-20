use std::collections::HashSet;

use crate::tolk::integration_tests::mutants_for_slug;

#[test]
fn as_swaps_function_arguments() {
    let source = r#"
fun call(a: int, b: int): int {
    return combine(a, b);
}

fun combine(x: int, y: int): int {
    return x + y;
}
"#;

    let mutants = mutants_for_slug(source, "AS");
    assert!(
        !mutants.is_empty(),
        "expected AS mutants to swap adjacent arguments"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    assert!(
        replacements.contains("b, a"),
        "expected swapped argument sequence 'b, a'; replacements: {replacements:?}"
    );
}
