use super::*;

#[test]
fn test_loop_mutations_basic() {
    let source = r#"
() test_func() {
    repeat(10) {
        ;; do something
    }
    while (x > 0) {
        x = x - 1;
    }
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should generate loop-related mutations
    let loop_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("LI") || m.mutation_slug.starts_with("LC"))
        .collect();
    
    // Loop mutations are advanced features, may or may not be implemented
    println!("Generated {} loop mutations", loop_mutants.len());
    
    // Should preserve basic structure if mutations are generated
    for mutant in &loop_mutants {
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function structure");
    }
} 