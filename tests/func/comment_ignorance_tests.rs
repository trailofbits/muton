use mewt::{mutations, types::Language};

use super::common::func_target;

#[test]
fn func_mutations_ignore_comment_regions() {
    let source = r#"()
main() {
    ;; if (true) { throw(1); }
    {- let y = 10; -}
    var x = 1;
    if (x > 0) { return x; }
}
"#;

    // NOTE: Keep this list in sync with source above.
    // Lines are 0-based and refer to fully-commented lines only.
    let commented_lines: &[usize] = &[2, 3];

    let fixture = func_target(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
    let mutants = engine.apply_all_mutations(fixture.target());

    // Ensure none of the mutants originate from commented content (line or block)
    for m in &mutants {
        let line = m.line_offset as usize;
        assert!(
            !commented_lines.contains(&line),
            "mutated on commented line: slug={} line={}",
            m.mutation_slug,
            line,
        );
    }

    // Ensure CR does not double-wrap block-commented content
    let cr_nested = mutants.iter().any(|m| m.mutation_slug == "CR" && m.new_text.contains("{- {-"));
    assert!(!cr_nested, "CR should not double-wrap commented content");
}


