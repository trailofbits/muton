use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_argument_swap_basic_function_call() {
    let source = r#"
() test_func() {
    var result = add(x, y);
    return result;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should find AS mutations that swap function arguments
    let as_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("AS"))
        .collect();
    
    assert!(!as_mutants.is_empty(), "Should generate AS mutations");
    
    // Check that arguments are swapped
    let has_swapped = as_mutants.iter()
        .any(|m| m.mutated_content.contains("add(y, x)"));
    assert!(has_swapped, "Should swap function arguments");
}

#[test]
fn test_argument_swap_multiple_arguments() {
    let source = r#"
() test_func() {
    var result = process(a, b, c, d);
    return result;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let as_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("AS"))
        .collect();
    
    assert!(!as_mutants.is_empty(), "Should generate AS mutations for multiple arguments");
    
    // Should generate multiple swap combinations
    assert!(as_mutants.len() >= 2, "Should generate multiple argument swap combinations");
    
    // Each mutation should have swapped at least one pair of adjacent arguments
    for mutant in &as_mutants {
        assert!(mutant.mutated_content.contains("process("), 
                "Should preserve function name");
        assert!(!mutant.mutated_content.contains("process(a, b, c, d)"), 
                "Should modify the original argument order");
    }
}

#[test]
fn test_argument_swap_chained_calls() {
    let source = r#"
() test_func() {
    var result = first(x, y).second(a, b);
    return result;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let as_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("AS"))
        .collect();
    
    // Should handle chained function calls appropriately
    for mutant in &as_mutants {
        // Should preserve the chaining structure
        assert!(mutant.mutated_content.contains("."), 
                "Should preserve method chaining");
        assert!(mutant.mutated_content.contains("first(") && mutant.mutated_content.contains("second("),
                "Should preserve both function calls");
    }
}

#[test]
fn test_argument_swap_single_argument() {
    let source = r#"
() test_func() {
    var result = single_arg(x);
    return result;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let as_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("AS"))
        .collect();
    
    // Should not generate AS mutations for single-argument functions
    // (or if it does, should leave the function unchanged)
    for mutant in &as_mutants {
        if mutant.mutated_content.contains("single_arg(x)") {
            // If targeting this line, should not change single argument calls
            assert_eq!(mutant.original_line.trim(), mutant.mutated_content.lines()
                      .find(|line| line.contains("single_arg"))
                      .unwrap_or("").trim(),
                      "Should not modify single argument function calls");
        }
    }
} 