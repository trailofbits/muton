use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn as_swaps_adjacent_arguments() {
    let source = r#"
() reorder() {
    call_all(foo(1, 2, 3));
}
"#;

    let mutants = mutants_for_slug(source, "AS");
    assert!(
        !mutants.is_empty(),
        "expected AS mutants to swap adjacent arguments"
    );

    let new_texts: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["2, 1", "3, 2"] {
        assert!(
            new_texts.contains(expected),
            "missing AS swapped argument `{expected}`; new_texts: {new_texts:?}"
        );
    }
}
