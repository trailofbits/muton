use crate::conformance;
use crate::utils;
use mewt::LanguageEngine;
use mewt::types::{Mutant, Target};
use muton::languages::func::engine::FuncLanguageEngine;

pub(crate) fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    utils::target_fixture_for_extension("FunC", "fc", content).into_parts()
}

pub(crate) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let (_tmp, target) = create_test_target(source);
    let engine = FuncLanguageEngine::new();
    utils::mutants_for_slug(&engine, &target, slug)
}

#[test]
fn func_common_conformance_checks() {
    let sources = conformance::CommonConformanceSources {
        basic_source: r#"
() test_func() {
    var x = 42;
    if (x > 0) {
        return x;
    }
    return 0;
}
"#,
        comment_source: r#"
() test_func() {
    ;; This is a comment
    var x = 42;
    if (x > 0) {
        return x;
    }
    return 0;
}
"#,
        complex_source: r#"
global int counter;

() recv_internal(int my_balance, int msg_value, cell in_msg_full, slice in_msg_body) impure {
    slice cs = in_msg_full.begin_parse();
    cs~skip_bits(4);

    slice sender_address = cs~load_msg_addr();

    if (msg_value > 0) {
        ;; Process message
        var result = process_message(sender_address, msg_value);
        if (result == 0) {
            return ();
        }
    }

    throw(0xffff);
}

int get_counter() method_id {
    return counter;
}
"#,
        line_coverage_source: r#"
() test_func() {
    var x = 42;
    var y = x + 1;
    if (x > 0) {
        return x;
    }
    return y;
}
"#,
    };

    let expectations = conformance::CommonConformanceExpectations {
        language_name: "FunC",
        comment_line_prefix: ";;",
        min_complex_mutants: 6,
    };

    conformance::run_common_language_checks(
        create_test_target,
        || Box::new(FuncLanguageEngine::new()),
        sources,
        expectations,
    );
}

#[test]
fn func_example_file_generates_mutants() {
    let source = conformance::read_example_source("tests/func/examples/hello-world.fc");
    let (_tmp, target) = create_test_target(&source);
    let mutants = FuncLanguageEngine::new().mutate(&target);

    assert!(
        !mutants.is_empty(),
        "FunC example file should generate mutants"
    );

    let mutated = target
        .mutate(&mutants[0])
        .expect("applying the first mutant should succeed");
    assert_ne!(mutated, target.text);
}

#[test]
fn func_shared_slugs_presence() {
    // FunC sample with if and a call with 2 args
    let func_src = r#"()
main() {
    var x = 1;
    if (x > 0) {
        return x;
    }
    foo(1, 2);
}
"#;

    let (_tmp, target) = create_test_target(func_src);
    let engine = FuncLanguageEngine::new();
    let mutants = engine.mutate(&target);

    fn count(mutants: &[mewt::types::Mutant], slug: &str) -> usize {
        mutants.iter().filter(|m| m.mutation_slug == slug).count()
    }

    let er_count = count(&mutants, "ER");
    let cr_count = count(&mutants, "CR");
    let as_count = count(&mutants, "AS");

    println!("func ER/CR/AS: {er_count}/{cr_count}/{as_count}");

    assert!(er_count > 0, "ER should be present in FunC");
    assert!(cr_count > 0, "CR should be present in FunC");
    // AS may or may not be present depending on implementation
}

#[test]
fn func_mutations_ignore_comment_regions() {
    let source = r#"()
main() {
    ;; if (true) { throw(1); }
    {- let y = 10; -}
    var x = 1;
    if (x > 0) { return x; }
}
"#;

    // NOTE: Keep this list in sync with source above.
    // Lines are 0-based and refer to fully-commented lines only.
    let commented_lines: &[usize] = &[2, 3];

    let (_tmp, target) = create_test_target(source);
    let engine = FuncLanguageEngine::new();
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
        .any(|m| m.mutation_slug == "CR" && m.new_text.contains("{- {-"));
    assert!(!cr_nested, "CR should not double-wrap commented content");
}
