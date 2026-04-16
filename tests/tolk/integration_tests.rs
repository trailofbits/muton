use std::collections::BTreeSet;

use mewt::{LanguageEngine, LanguageRegistry};
use muton::languages::tolk::engine::TolkLanguageEngine;

use super::common::{slug_set, tolk_target};

#[test]
fn generates_mutants_for_control_flow_and_expressions() {
    let source = r#"
fun compute(a: int, b: int): int {
    var result = a + b;
    var flag = true;

    if (a > b) {
        result = result - 1;
    }

    while (result < 42) {
        result = result + 1;
    }

    return result & (a | b);
}
"#;

    let fixture = tolk_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.mutate(fixture.target());

    assert!(
        !mutants.is_empty(),
        "expected at least one mutant for a non-trivial Tolk program"
    );

    let mutation_slugs = slug_set(&mutants);
    for expected in ["AOS", "BL", "IF", "WF"] {
        assert!(
            mutation_slugs.contains(expected),
            "expected mutation slug `{expected}` in generated mutants; slugs: {mutation_slugs:?}"
        );
    }
}

#[test]
fn language_registry_end_to_end_flow() {
    let source = r#"
fun main(): int {
    var value = 3;
    if (value == 3) {
        value = value + 1;
    }
    return value;
}
"#;

    let fixture = tolk_target(source);
    let mut registry = LanguageRegistry::new();
    registry.register(TolkLanguageEngine::new());

    let target = fixture.target();
    let mutants = target
        .generate_mutants(&registry)
        .expect("mutant generation to succeed");
    assert!(
        !mutants.is_empty(),
        "registry-backed mutation generation should yield mutants"
    );

    let mutated = target
        .mutate(&mutants[0])
        .expect("applying the first mutant should succeed");
    assert_ne!(
        mutated, target.text,
        "applying a mutant should alter the source text"
    );
}

#[test]
fn engine_reports_expected_metadata_and_slugs() {
    let engine = TolkLanguageEngine::new();

    assert_eq!(engine.name(), "Tolk");
    assert_eq!(engine.extensions(), &["tolk"]);

    let slugs: BTreeSet<_> = engine
        .get_mutations()
        .iter()
        .map(|m| m.slug)
        .collect();

    let expected: BTreeSet<_> = [
        "AAOS", "AOS", "AS", "BAOS", "BL", "BOS", "COS", "CR", "ER", "IF",
        "IT", "LC", "LOS", "SAOS", "SOS", "WF",
    ]
    .into_iter()
    .collect();

    assert_eq!(
        slugs, expected,
        "unexpected mutation slugs advertised by the Tolk engine"
    );
}
