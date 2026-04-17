use std::collections::HashSet;

use crate::func::integration_tests::mutants_for_slug;

#[test]
fn lc_swaps_loop_control_statements() {
    let source = r#"
() iterate() {
    while (true) {
        break;
        continue;
    }
}
"#;

    let mutants = mutants_for_slug(source, "LC");
    assert!(
        !mutants.is_empty(),
        "expected LC mutants to swap loop control statements"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["break", "continue"] {
        assert!(
            replacements.contains(expected),
            "missing LC replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
