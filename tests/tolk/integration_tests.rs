use std::collections::BTreeSet;

use crate::conformance;
use crate::utils;
use mewt::types::{Mutant, Target};
use mewt::{LanguageEngine, LanguageRegistry};
use muton::languages::tolk::engine::TolkLanguageEngine;

pub(crate) fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    utils::target_fixture_for_extension("Tolk", "tolk", content).into_parts()
}

pub(crate) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let (_tmp, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    utils::mutants_for_slug(&engine, &target, slug)
}

pub(crate) fn first_mutated_source(source: &str, slug: &str) -> Option<String> {
    let (_tmp, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mut mutants = utils::mutants_for_slug(&engine, &target, slug);
    utils::sort_by_byte_offset(&mut mutants);
    mutants
        .into_iter()
        .next()
        .and_then(|m| target.mutate(&m).ok())
}

#[test]
fn tolk_common_conformance_checks() {
    let sources = conformance::CommonConformanceSources {
        basic_source: r#"
fun main(): int {
    var value = 3;
    if (value == 3) {
        value = value + 1;
    }
    return value;
}
"#,
        comment_source: r#"
fun main(): int {
    // This is a comment
    var value = 3;
    if (value == 3) {
        value = value + 1;
    }
    return value;
}
"#,
        complex_source: r#"
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
"#,
        line_coverage_source: r#"
fun flow(a: int, b: int): int {
    var x = a + b;
    if (x > 0) {
        return x;
    }
    return b;
}
"#,
    };

    let expectations = conformance::CommonConformanceExpectations {
        language_name: "Tolk",
        comment_line_prefix: "//",
        min_complex_mutants: 6,
    };

    conformance::run_common_language_checks(
        create_test_target,
        || Box::new(TolkLanguageEngine::new()),
        sources,
        expectations,
    );
}

#[test]
fn tolk_example_file_generates_mutants() {
    let source = conformance::read_example_source("tests/tolk/examples/hello-world.tolk");
    let (_tmp, target) = create_test_target(&source);
    let mutants = TolkLanguageEngine::new().mutate(&target);

    assert!(
        !mutants.is_empty(),
        "Tolk example file should generate mutants"
    );

    let mutated = target
        .mutate(&mutants[0])
        .expect("applying the first mutant should succeed");
    assert_ne!(mutated, target.text);
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

    let (_tmp, target) = create_test_target(source);
    let mut registry = LanguageRegistry::new();
    registry.register(TolkLanguageEngine::new());

    let mutants = target
        .generate_mutants(&registry, None)
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

    let slugs: BTreeSet<_> = engine.get_mutations().iter().map(|m| m.slug).collect();

    let expected: BTreeSet<_> = [
        "AAOS", "AOS", "AS", "BAOS", "BL", "BOS", "COS", "CR", "ER", "IF", "IT", "LC", "LOS", "NR",
        "SAOS", "SOS", "WF",
    ]
    .into_iter()
    .collect();

    assert_eq!(
        slugs, expected,
        "unexpected mutation slugs advertised by the Tolk engine"
    );
}
