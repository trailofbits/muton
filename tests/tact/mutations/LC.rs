use std::collections::HashSet;

use super::mutants_for_slug;

#[test]
fn lc_swaps_loop_control_statements() {
    let source = r#"
    contract Loop {
        fun f(flag: Bool) {
            while (true) {
                if (flag) {
                    break;
                }
                continue;
            }
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "LC");
    assert!(
        !mutants.is_empty(),
        "expected LC mutants to swap break/continue"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    for expected in ["continue", "break"] {
        assert!(
            replacements.contains(expected),
            "missing LC replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
