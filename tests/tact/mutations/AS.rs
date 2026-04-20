use std::collections::HashSet;

use crate::tact::integration_tests::mutants_for_slug;

#[test]
fn as_swaps_arguments_on_calls() {
    let source = r#"
    contract D {
        fun f(a: Int, b: Int) {
            this.doIt(a, b);
            Math::mul(a, b);
        }
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
