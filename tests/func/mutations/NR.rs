use crate::func::integration_tests::mutants_for_slug;

#[test]
fn nr_is_handled_without_panicking() {
    let source = r#"
() check(int value) {
    if (value > 0) {
        return ();
    }
}
"#;

    // Sanity-check the source parses/mutates normally.
    let if_mutants = mutants_for_slug(source, "IF");
    assert!(
        !if_mutants.is_empty(),
        "expected IF mutants for valid source"
    );

    // FunC currently has no `!expr` unary form exposed for mutation by this engine,
    // so NR is expected to be a no-op instead of causing an unknown-slug panic.
    let nr_mutants = mutants_for_slug(source, "NR");
    assert!(
        nr_mutants.is_empty(),
        "expected NR to be a no-op for this FunC source"
    );
}
