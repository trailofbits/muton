use muton::languages::func::engine::FuncLanguageEngine;
use mewt::types::{Target, Hash};
use std::collections::{HashMap, HashSet};
use tempfile::tempdir;

/// Helper to create test target
fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.fc");
    std::fs::write(&file_path, content).expect("Failed to write test file");
    let target = Target {
        id: 1,
        path: file_path,
        file_hash: Hash::digest(content.to_string()),
        text: content.to_string(),
        language: "func".to_string(),
    };
    (temp_dir, target)
}

#[test]
fn test_mutation_count_comparison() {
    let source = r#"
() test_func() {
    var x = 42;
    if (x > 0) {
        return x;
    }
    return 0;
}
"#;

    let (_temp_dir, target) = create_test_target(source);

    // Get AST mutations
    let ast_engine = FuncLanguageEngine::new();
    let ast_mutants = ast_engine.apply_all_mutations(&target);

    println!("AST mutations: {}", ast_mutants.len());

    // AST should generate reasonable number of mutations
    assert!(
        !ast_mutants.is_empty(),
        "AST should generate some mutations"
    );

    // Check mutation types
    let ast_slugs: HashSet<_> = ast_mutants
        .iter()
        .map(|m| m.mutation_slug.chars().take(2).collect::<String>())
        .collect();

    println!("AST mutation types: {ast_slugs:?}");

    // Should generate diverse mutation types
    assert!(
        ast_slugs.len() > 3,
        "AST should generate diverse mutation types"
    );
}

#[test]
fn test_mutation_quality_comparison() {
    let source = r#"
() test_func() {
    ;; This is a comment
    var x = 42;
    if (x > 0) {
        return x;
    }
    return 0;
}
"#;

    let (_temp_dir, target) = create_test_target(source);

    // Get AST mutations
    let ast_engine = FuncLanguageEngine::new();
    let ast_mutants = ast_engine.apply_all_mutations(&target);

    // Check comment handling (checking old_text for comment patterns)
    let ast_comment_mutations = ast_mutants
        .iter()
        .filter(|m| m.old_text.trim().starts_with(";;"))
        .count();

    println!("AST comment mutations: {ast_comment_mutations}");

    // AST should avoid mutating comment-only lines
    assert_eq!(
        ast_comment_mutations, 0,
        "AST should not mutate comment-only lines"
    );
}

#[test]
fn test_complex_code_handling() {
    let source = r#"
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
"#;

    let (_temp_dir, target) = create_test_target(source);

    // Test that AST system can handle complex FunC code
    let ast_engine = FuncLanguageEngine::new();
    let ast_result = std::panic::catch_unwind(|| ast_engine.apply_all_mutations(&target));

    assert!(
        ast_result.is_ok(),
        "AST system should handle complex code without panicking"
    );

    if let Ok(ast_mutants) = ast_result {
        println!("Complex code - AST mutations: {}", ast_mutants.len());

        // Should generate substantial mutations for complex code
        assert!(
            ast_mutants.len() > 5,
            "AST should generate substantial mutations for complex code"
        );
    }
}

#[test]
fn test_mutation_overlap_analysis() {
    let source = r#"
() test_func() {
    var x = 42;
    var y = x + 1;
    if (x > 0) {
        return x;
    }
    return y;
}
"#;

    let (_temp_dir, target) = create_test_target(source);

    let ast_engine = FuncLanguageEngine::new();
    let ast_mutants = ast_engine.apply_all_mutations(&target);

    // Analyze which lines are affected by mutations
    let mut ast_lines: HashMap<usize, Vec<String>> = HashMap::new();

    for mutant in &ast_mutants {
        ast_lines
            .entry(mutant.line_offset as usize)
            .or_default()
            .push(mutant.mutation_slug.clone());
    }

    println!("AST mutations by line: {ast_lines:?}");

    // Should affect multiple lines for decent coverage
    assert!(
        ast_lines.len() > 1,
        "AST mutations should affect multiple lines"
    );
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
    
    let (_temp_dir, target) = create_test_target(func_src);
    let engine = FuncLanguageEngine::new();
    let mutants = engine.apply_all_mutations(&target);

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