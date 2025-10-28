use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_error_replacement_basic_statement() {
    let source = r#"
() test_func() {
    var x = 42;
    return x;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find ER mutations that replace statements with throw(1)
    let er_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("ER"))
        .collect();
    
    assert!(!er_mutants.is_empty(), "Should generate ER mutations");
    
    // Check that throw(1) replacement is present
    let has_throw = er_mutants.iter()
        .any(|m| m.mutated_content.contains("throw(1)"));
    assert!(has_throw, "Should replace statements with throw(1)");
}

#[test]
fn test_error_replacement_ignores_comments() {
    let source = r#"
() test_func() {
    ;; This is a comment
    var x = 42;  ;; Another comment
    return x;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // ER mutations should not target comment lines
    let er_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("ER"))
        .collect();
    
    // Should not mutate lines that are only comments
    let comment_mutations = er_mutants.iter()
        .filter(|m| m.original_line.trim().starts_with(";;"))
        .count();
    
    assert_eq!(comment_mutations, 0, "Should not mutate comment-only lines");
}

#[test]
fn test_error_replacement_multiple_statements() {
    let source = r#"
() test_func() {
    var a = 1;
    var b = 2;
    var c = a + b;
    return c;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let er_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("ER"))
        .collect();
    
    // Should generate multiple ER mutations for each statement
    assert!(er_mutants.len() >= 3, "Should generate ER mutations for multiple statements");
    
    // Each mutation should preserve other statements while replacing one
    for mutant in &er_mutants {
        assert!(mutant.mutated_content.contains("throw(1)"), 
                "Each ER mutation should contain throw(1)");
        
        // Should preserve function structure
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function signature");
    }
}

#[test]
fn test_error_replacement_preserves_structure() {
    let source = r#"
() test_func() {
    if (true) {
        var x = 42;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let er_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("ER"))
        .collect();
    
    assert!(!er_mutants.is_empty(), "Should generate ER mutations");
    
    // Verify that structure is preserved
    for mutant in &er_mutants {
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function structure");
        assert!(mutant.mutated_content.contains("{") && mutant.mutated_content.contains("}"),
                "Should preserve braces structure");
    }
} 