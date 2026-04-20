use std::collections::HashSet;

use crate::tact::integration_tests::mutants_for_slug;

#[test]
fn los_shuffles_logical_operators() {
    let source = r#"
    contract K {
        fun f(a: Bool, b: Bool): Bool {
            return a && b;
        }
    }
    "#;

    let mutants = mutants_for_slug(source, "LOS");
    assert!(
        !mutants.is_empty(),
        "expected LOS mutants to shuffle logical operators"
    );

    let replacements: HashSet<_> = mutants
        .iter()
        .map(|m| m.new_text.trim().to_string())
        .collect();
    {
        let expected = "||";
        assert!(
            replacements.contains(expected),
            "missing LOS replacement `{expected}`; replacements: {replacements:?}"
        );
    }
}
