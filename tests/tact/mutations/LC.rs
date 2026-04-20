use std::collections::HashSet;

use crate::tact::integration_tests::mutants_for_slug;

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
    let saw_continue = replacements.contains("continue") || replacements.contains("continue;");
    let saw_break = replacements.contains("break") || replacements.contains("break;");
    assert!(
        saw_continue && saw_break,
        "missing LC replacements; replacements: {replacements:?}"
    );
}
