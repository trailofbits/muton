use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_comment_replacement_basic_statement() {
    let source = r#"
() test_func() {
    var x = 42;
    return x;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find CR mutations that comment out statements
    let cr_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("CR"))
        .collect();
    
    assert!(!cr_mutants.is_empty(), "Should generate CR mutations");
    
    // Check that statements are commented out with ;;
    let has_comment = cr_mutants.iter()
        .any(|m| m.mutated_content.contains(";;"));
    assert!(has_comment, "Should comment out statements with ;;");
}

#[test]
fn test_comment_replacement_preserves_existing_comments() {
    let source = r#"
() test_func() {
    ;; Existing comment
    var x = 42;
    return x;  ;; Inline comment
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let cr_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("CR"))
        .collect();
    
    // Should preserve existing comments when commenting out code
    for mutant in &cr_mutants {
        assert!(mutant.mutated_content.contains(";; Existing comment"), 
                "Should preserve existing standalone comments");
    }
}

#[test]
fn test_comment_replacement_multiple_statements() {
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
    
    let cr_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("CR"))
        .collect();
    
    // Should generate CR mutations for each statement
    assert!(cr_mutants.len() >= 3, "Should generate CR mutations for multiple statements");
    
    // Each mutation should comment out one statement while preserving others
    for mutant in &cr_mutants {
        // Should have at least one commented line
        let commented_lines = mutant.mutated_content.lines()
            .filter(|line| line.trim().starts_with(";;"))
            .count();
        assert!(commented_lines >= 1, "Should have at least one commented line");
        
        // Should preserve function structure
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function signature");
    }
}

#[test]
fn test_comment_replacement_doesnt_double_comment() {
    let source = r#"
() test_func() {
    ;; Already commented
    var x = 42;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let cr_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("CR"))
        .collect();
    
    // Should not try to comment already commented lines
    for mutant in &cr_mutants {
        // Should not have lines that start with ";; ;;"
        let double_commented = mutant.mutated_content.lines()
            .any(|line| line.trim().starts_with(";; ;;"));
        assert!(!double_commented, "Should not double-comment already commented lines");
    }
} 