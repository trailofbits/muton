use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_malformed_code_handling() {
    let malformed_sources = vec![
        // Missing closing brace
        r#"() test_func() { var x = 42;"#,
        
        // Invalid syntax
        r#"() test_func() { var x = ; }"#,
        
        // Incomplete statements
        r#"() test_func() { var }"#,
        
        // Empty file
        r#""#,
    ];
    
    for (i, source) in malformed_sources.iter().enumerate() {
        let (_temp_dir, target) = create_test_target(source);
        let engine = get_ast_engine();
        
        // Should not panic on malformed code
        let result = std::panic::catch_unwind(|| {
            engine.apply_all_mutations(&target)
        });
        
        assert!(result.is_ok(), "Should not panic on malformed code {}", i);
        
        // Should either return empty mutations or gracefully handle errors
        if let Ok(mutants) = result {
            // Malformed code should typically result in no mutations
            // or the system should gracefully fall back
            println!("Malformed case {}: generated {} mutations", i, mutants.len());
        }
    }
}

#[test]
fn test_unicode_handling() {
    let source = r#"
() test_func() {
    ;; Unicode comment: 测试 🚀 Ü
    var x = "Hello 世界";
    return x;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should handle Unicode characters properly
    assert!(!mutants.is_empty(), "Should generate mutations for Unicode content");
    
    for mutant in &mutants {
        // Should preserve Unicode characters
        if mutant.original_line.contains("Unicode comment") {
            assert!(mutant.mutated_content.contains("测试") || 
                   mutant.mutated_content.contains("🚀") ||
                   mutant.mutated_content.contains("Ü"),
                   "Should preserve Unicode characters in mutations");
        }
        
        // Mutated content should be valid UTF-8
        assert!(std::str::from_utf8(mutant.mutated_content.as_bytes()).is_ok(),
                "Mutated content should be valid UTF-8");
    }
}

#[test]
fn test_large_file_handling() {
    // Generate a large FunC file with many functions
    let mut large_source = String::new();
    large_source.push_str("global int counter;\n\n");
    
    for i in 0..100 {
        large_source.push_str(&format!(
            r#"() func_{i}() {{
    var x_{i} = {i};
    var y_{i} = x_{i} + 1;
    if (y_{i} > 0) {{
        return y_{i};
    }}
    return 0;
}}

"#, i = i));
    }
    
    let (_temp_dir, target) = create_test_target(&large_source);
    let engine = get_ast_engine();
    
    let start_time = std::time::Instant::now();
    let mutants = engine.apply_all_mutations(&target);
    let duration = start_time.elapsed();
    
    // Should handle large files within reasonable time
    assert!(duration.as_secs() < 10, "Should process large files within 10 seconds");
    
    // Should generate a reasonable number of mutations
    assert!(mutants.len() > 100, "Should generate substantial mutations for large files");
    assert!(mutants.len() < 10000, "Should not generate excessive mutations");
    
    println!("Large file test: {} mutations in {:?}", mutants.len(), duration);
}

#[test]
fn test_deeply_nested_structures() {
    let source = r#"
() test_func() {
    if (a > 0) {
        if (b > 0) {
            if (c > 0) {
                if (d > 0) {
                    if (e > 0) {
                        return e;
                    }
                }
            }
        }
    }
    return 0;
}
"#;
    
    let (_temp_dir, target) = create_test_target(source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should handle deeply nested structures
    assert!(!mutants.is_empty(), "Should handle deeply nested structures");
    
    // Should generate appropriate mutations for each level
    let conditional_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.mutation_slug.starts_with("IF") || m.mutation_slug.starts_with("IT"))
        .collect();
    
    assert!(conditional_mutants.len() >= 5, "Should generate mutations for each nested condition");
}

#[test]
fn test_very_long_lines() {
    let long_line = format!("var x = {};", "a + ".repeat(1000));
    let source = format!(r#"
() test_func() {{
    {long_line}
    return x;
}}
"#, long_line = long_line);
    
    let (_temp_dir, target) = create_test_target(&source);
    let engine = get_ast_engine();
    
    let mutants = engine.apply_all_mutations(&target);
    
    // Should handle very long lines without issues
    assert!(!mutants.is_empty(), "Should handle very long lines");
    
    for mutant in &mutants {
        // Should not corrupt long lines
        assert!(mutant.mutated_content.len() > 0, "Should not produce empty mutations");
        
        // Should preserve function structure even with long lines
        assert!(mutant.mutated_content.contains("() test_func()"), 
                "Should preserve function structure with long lines");
    }
} 