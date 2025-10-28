use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_if_false_mutation() {
    let source = r#"
() test_func() {
    if (x > 0) {
        return x;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find IF mutations that replace conditions with false
    let if_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("IF"))
        .collect();
    
    assert!(!if_mutants.is_empty(), "Should generate IF mutations");
    
    // Check that condition is replaced with false
    let has_false = if_mutants.iter()
        .any(|m| m.mutated_content.contains("if (false)"));
    assert!(has_false, "Should replace if condition with false");
}

#[test]
fn test_if_true_mutation() {
    let source = r#"
() test_func() {
    if (x > 0) {
        return x;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find IT mutations that replace conditions with true
    let it_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("IT"))
        .collect();
    
    assert!(!it_mutants.is_empty(), "Should generate IT mutations");
    
    // Check that condition is replaced with true
    let has_true = it_mutants.iter()
        .any(|m| m.mutated_content.contains("if (true)"));
    assert!(has_true, "Should replace if condition with true");
}

#[test]
fn test_ifnot_false_mutation() {
    let source = r#"
() test_func() {
    ifnot (x > 0) {
        return 0;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find INF mutations for ifnot statements
    let inf_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("INF"))
        .collect();
    
    assert!(!inf_mutants.is_empty(), "Should generate INF mutations");
    
    // Check that ifnot condition is replaced with false
    let has_false = inf_mutants.iter()
        .any(|m| m.mutated_content.contains("ifnot (false)"));
    assert!(has_false, "Should replace ifnot condition with false");
}

#[test]
fn test_ifnot_true_mutation() {
    let source = r#"
() test_func() {
    ifnot (x > 0) {
        return 0;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find INT mutations for ifnot statements
    let int_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("INT"))
        .collect();
    
    assert!(!int_mutants.is_empty(), "Should generate INT mutations");
    
    // Check that ifnot condition is replaced with true
    let has_true = int_mutants.iter()
        .any(|m| m.mutated_content.contains("ifnot (true)"));
    assert!(has_true, "Should replace ifnot condition with true");
}

#[test]
fn test_complex_conditions() {
    let source = r#"
() test_func() {
    if (x > 0 && y < 10) {
        return x + y;
    }
    ifnot (z == 0 || w != 5) {
        return z * w;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let conditional_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("IF") || m.mutation_slug.starts_with("IT") || 
                    m.mutation_slug.starts_with("INF") || m.mutation_slug.starts_with("INT"))
        .collect();
    
    assert!(!conditional_mutants.is_empty(), "Should generate conditional mutations");
    
    // Should handle complex conditions appropriately
    for mutant in &conditional_mutants {
        // Should preserve function structure
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function signature");
        
        // Should have valid boolean replacement
        assert!(mutant.mutated_content.contains("true") || mutant.mutated_content.contains("false"),
                "Should contain boolean replacement");
    }
} 