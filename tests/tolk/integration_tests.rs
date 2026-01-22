use muton::languages::tolk::engine::TolkLanguageEngine;
use mewt::types::{Target, Hash};
use mewt::LanguageEngine;
use std::collections::HashSet;
use tempfile::tempdir;

/// Helper to create test target
fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.tolk");
    std::fs::write(&file_path, content).expect("Failed to write test file");
    let target = Target {
        id: 1,
        path: file_path,
        file_hash: Hash::digest(content.to_string()),
        text: content.to_string(),
        language: "Tolk".to_string(),
    };
    (temp_dir, target)
}

#[test]
fn test_basic_mutations() {
    let source = r#"
fun test_func(x: int): int {
    if (x > 0) {
        return x;
    }
    return 0;
}
"#;

    let (_temp_dir, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

    println!("Generated {} mutations", mutants.len());

    // Should generate some mutations
    assert!(
        !mutants.is_empty(),
        "Should generate mutations for basic Tolk code"
    );

    // Check mutation types
    let mutation_slugs: HashSet<_> = mutants
        .iter()
        .map(|m| m.mutation_slug.as_str())
        .collect();

    println!("Mutation types: {mutation_slugs:?}");

    // Should generate diverse mutation types (IF, IT, ER, CR, COS, etc.)
    assert!(
        mutation_slugs.len() > 2,
        "Should generate diverse mutation types"
    );
}

#[test]
fn test_if_statement_mutations() {
    let source = r#"
fun check(val: bool): int {
    if (val) {
        return 1;
    }
    return 0;
}
"#;

    let (_temp_dir, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

    // Should have IF and IT mutations
    let if_mutants: Vec<_> = mutants
        .iter()
        .filter(|m| m.mutation_slug == "IF" || m.mutation_slug == "IT")
        .collect();

    assert!(
        !if_mutants.is_empty(),
        "Should generate IF/IT mutations for if statements"
    );
}

#[test]
fn test_operator_mutations() {
    let source = r#"
fun calculate(a: int, b: int): int {
    var sum = a + b;
    var diff = a - b;
    var prod = a * b;
    return sum;
}
"#;

    let (_temp_dir, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

    // Should have arithmetic operator mutations (AOS)
    let aos_mutants: Vec<_> = mutants
        .iter()
        .filter(|m| m.mutation_slug == "AOS")
        .collect();

    assert!(
        !aos_mutants.is_empty(),
        "Should generate AOS mutations for arithmetic operators"
    );
}

#[test]
fn test_boolean_literal_mutations() {
    let source = r#"
fun isActive(): bool {
    var active = true;
    var inactive = false;
    return active;
}
"#;

    let (_temp_dir, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

    // Should have boolean literal mutations (BL)
    let bl_mutants: Vec<_> = mutants
        .iter()
        .filter(|m| m.mutation_slug == "BL")
        .collect();

    assert!(
        !bl_mutants.is_empty(),
        "Should generate BL mutations for boolean literals"
    );
}

#[test]
fn test_while_loop_mutations() {
    let source = r#"
fun loop_test(n: int): int {
    var i = 0;
    while (i < n) {
        i = i + 1;
    }
    return i;
}
"#;

    let (_temp_dir, target) = create_test_target(source);
    let engine = TolkLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

    // Should have while false mutations (WF)
    let wf_mutants: Vec<_> = mutants
        .iter()
        .filter(|m| m.mutation_slug == "WF")
        .collect();

    assert!(
        !wf_mutants.is_empty(),
        "Should generate WF mutations for while loops"
    );
}

#[test]
fn test_engine_properties() {
    let engine = TolkLanguageEngine::new();

    assert_eq!(engine.name(), "Tolk");
    assert_eq!(engine.extensions(), &["tolk"]);
    assert!(!engine.get_mutations().is_empty());
}
