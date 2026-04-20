use crate::tact::integration_tests::first_mutated_source;

#[test]
fn nr_removes_logical_negation() {
    let source = r#"
    contract C {
        fun f(flag: Bool) {
            if (!flag) {
                return;
            }
        }
    }
    "#;

    let mutated = first_mutated_source(source, "NR").expect("NR mutant");
    assert!(
        mutated.contains("if (flag)"),
        "expected NR mutant to remove `!` from condition; mutated source: {mutated}"
    );
}
