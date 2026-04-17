use crate::conformance;
use crate::utils;
use mewt::types::{Mutant, Target};
use mewt::{LanguageEngine, LanguageRegistry};
use muton::languages::tact::engine::TactLanguageEngine;

pub(crate) fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    utils::target_fixture_for_extension("Tact", "tact", content).into_parts()
}

pub(crate) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let (_tmp, target) = create_test_target(source);
    let engine = TactLanguageEngine::new();
    utils::mutants_for_slug(&engine, &target, slug)
}

pub(crate) fn first_mutated_source(source: &str, slug: &str) -> Option<String> {
    let (_tmp, target) = create_test_target(source);
    let engine = TactLanguageEngine::new();
    let mut mutants = utils::mutants_for_slug(&engine, &target, slug);
    utils::sort_by_byte_offset(&mut mutants);
    mutants
        .into_iter()
        .next()
        .and_then(|m| target.mutate(&m).ok())
}

#[test]
fn tact_common_conformance_checks() {
    let sources = conformance::CommonConformanceSources {
        basic_source: r#"
contract G {
    fun f(a: Int, b: Int): Int {
        if (a != b) {
            return a;
        }
        return b;
    }
}
"#,
        comment_source: r#"
contract C {
    fun f(a: Int, b: Int): Int {
        // This is a comment
        if (a != b) {
            return a;
        }
        return b;
    }
}
"#,
        complex_source: r#"
contract Counter {
    let value: Int = 0;

    fun process(a: Int, b: Int): Int {
        let sum: Int = a + b;
        if (sum > 10) {
            return sum;
        }
        let adjusted: Int = sum * 2;
        while (adjusted > 0) {
            return adjusted;
        }
        return 0;
    }
}
"#,
        line_coverage_source: r#"
contract Flow {
    fun test(a: Int, b: Int): Int {
        let x: Int = a + b;
        if (x > 0) {
            return x;
        }
        return b;
    }
}
"#,
    };

    let expectations = conformance::CommonConformanceExpectations {
        language_name: "Tact",
        comment_line_prefix: "//",
        min_complex_mutants: 6,
    };

    conformance::run_common_language_checks(
        create_test_target,
        || Box::new(TactLanguageEngine::new()),
        sources,
        expectations,
    );
}

#[test]
fn tact_example_file_generates_mutants() {
    let source = conformance::read_example_source("tests/tact/examples/hello-world.tact");
    let (_tmp, target) = create_test_target(&source);
    let mutants = TactLanguageEngine::new().mutate(&target);

    assert!(
        !mutants.is_empty(),
        "Tact example file should generate mutants"
    );

    let mutated = target
        .mutate(&mutants[0])
        .expect("applying the first mutant should succeed");
    assert_ne!(mutated, target.text);
}

#[test]
fn end_to_end_generate_mutants_tact() {
    let source = r#"
	contract G {
		fun f(a: Int, b: Int): Int {
			if (a != b) {
				return a;
			}
			return b;
		}
	}
	"#;
    let (_tmp, target) = create_test_target(source);

    // Create a language registry with the Tact engine
    let mut registry = LanguageRegistry::new();
    registry.register(TactLanguageEngine::new());

    let mutants = target.generate_mutants(&registry, None).expect("mutants");
    assert!(mutants.len() > 0, "expected at least one mutant");

    // Mutating content should succeed
    let mutated = target.mutate(&mutants[0]).expect("mutated content");
    assert_ne!(mutated, target.text);
}

#[test]
fn tact_mutations_ignore_comment_regions() {
    let source = r#"
// if (true) { require(false); }
// let x: Int = 1 + 2;
// if (1 < 2) { let y: Int = 3; }
// this.callMe(10, 20);
// while (true) { break; }
contract C {
    init() { }
    receive("hello") { }
    fun f(a: Int, b: Int): Int {
        return a + b;
    }
}
"#;

    // NOTE: Keep this list in sync with source above.
    // Lines are 0-based and refer to fully-commented lines only.
    let commented_lines: &[usize] = &[1, 2, 3, 4, 5];

    let (_tmp, target) = create_test_target(source);
    let engine = TactLanguageEngine::new();
    let mutants = engine.mutate(&target);

    // Ensure none of the mutants originate from commented content (line or block)
    for m in &mutants {
        let line = m.line_offset as usize;
        assert!(
            !commented_lines.contains(&line),
            "mutated on commented line: slug={} line={}",
            m.mutation_slug,
            line,
        );
    }

    // Ensure CR does not double-wrap block-commented content
    let cr_nested = mutants
        .iter()
        .any(|m| m.mutation_slug == "CR" && m.new_text.contains("/* /*"));
    assert!(!cr_nested, "CR should not double-wrap commented content");
}
