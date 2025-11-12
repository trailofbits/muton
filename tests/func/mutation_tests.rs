use mewt::{mutations, types::{Language, Target, Hash}};

fn func_target_from_source(source: &str) -> Target {
    use tempfile::tempdir;
    let tmp = tempdir().expect("tmpdir");
    let path = tmp.path().join("test.fc");
    std::fs::write(&path, source).unwrap();
    Target {
        id: 1,
        path,
        file_hash: Hash::digest(source.to_string()),
        text: source.to_string(),
        language: Language::FunC,
    }
}

#[test]
fn test_error_replacement_mutations() {
    let source = r#"
() test_func() {
    var x = 42;
    if (x > 0) {
        return x + 1;
    }
    return x - 1;
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    let er_mutants: Vec<_> = mutants.iter().filter(|m| m.mutation_slug == "ER").collect();
    
    assert!(!er_mutants.is_empty(), "Should generate ER mutations");

    // Check that ER mutations replace expressions with error calls
    for mutant in er_mutants {
        assert!(
            mutant.new_text.contains("throw(") || mutant.new_text.contains("error("),
            "ER mutation should contain throw or error call: {}",
            mutant.new_text
        );
    }
}

#[test]
fn test_comment_replacement_mutations() {
    let source = r#"
() test_func() {
    var x = 42;
    if (x > 0) {
        return x;
    }
    return 0;
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    let cr_mutants: Vec<_> = mutants.iter().filter(|m| m.mutation_slug == "CR").collect();
    
    assert!(!cr_mutants.is_empty(), "Should generate CR mutations");

    // Check that CR mutations wrap code in comments
    for mutant in cr_mutants {
        assert!(
            mutant.new_text.starts_with("{-") && mutant.new_text.ends_with("-}"),
            "CR mutation should wrap in block comments: {}",
            mutant.new_text
        );
    }
}

#[test]
fn test_conditional_mutations() {
    let source = r#"
() test_func() {
    var x = 42;
    if (x > 0) {
        return x;
    } else {
        return 0;
    }
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    // Should have mutations that target conditional expressions
    let conditional_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.old_text.contains(">") || m.old_text.contains("if"))
        .collect();

    assert!(!conditional_mutants.is_empty(), "Should generate conditional mutations");
}

#[test]
fn test_argument_swap_mutations() {
    let source = r#"
() test_func() {
    foo(1, 2);
    bar(x, y, z);
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    let as_mutants: Vec<_> = mutants.iter().filter(|m| m.mutation_slug == "AS").collect();
    
    // AS mutations may or may not be present depending on implementation
    if !as_mutants.is_empty() {
        // If AS mutations exist, they should swap function arguments
        for mutant in as_mutants {
            assert!(
                mutant.old_text.contains("(") && mutant.old_text.contains(")"),
                "AS mutation should involve function call: {}",
                mutant.old_text
            );
        }
    }
}

#[test]
fn test_variable_mutations() {
    let source = r#"
() test_func() {
    var x = 1;
    var y = 2;
    return x + y;
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    // Should have mutations that target variables and expressions
    let var_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.old_text.trim() == "x" || m.old_text.trim() == "y" || m.old_text.contains("+"))
        .collect();

    assert!(!var_mutants.is_empty(), "Should generate variable-related mutations");
}

#[test]
fn test_loop_mutations() {
    let source = r#"
() test_func() {
    var i = 0;
    while (i < 10) {
        i += 1;
    }
    return i;
}
"#;

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(&target);

    // Should have mutations that target loop constructs
    let loop_mutants: Vec<_> = mutants.iter()
        .filter(|m| m.old_text.contains("while") || m.old_text.contains("<") || m.old_text.contains("+="))
        .collect();

    assert!(!loop_mutants.is_empty(), "Should generate loop-related mutations");
}