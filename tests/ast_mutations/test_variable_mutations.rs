use super::*;

#[test]
fn test_variable_mutations_basic() {
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
    
    // Should generate variable-related mutations
    let var_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("VS") || m.mutation_slug.starts_with("VI"))
        .collect();
    
    assert!(!var_mutants.is_empty(), "Should generate variable mutations");
    
    // Should preserve basic structure
    for mutant in &var_mutants {
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function structure");
    }
}

#[test]
fn test_variable_scope_awareness() {
    let source = r#"
() test_func() {
    var outer = 1;
    if (true) {
        var inner = 2;
        return inner + outer;
    }
    return outer;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    let vs_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("VS"))
        .collect();
    
    // Variable swap mutations should be scope-aware
    // (Implementation details depend on actual AST mutation logic)
    assert!(!vs_mutants.is_empty(), "Should generate variable swap mutations");
} 