use muton::languages::tact::engine::TactLanguageEngine;

use super::common::tact_target;

#[test]
fn tact_mutations_ignore_comment_regions() {
    let source = r#"
// if (true) { require(false); }
// let x: Int = 1 + 2;
// if (1 < 2) { let y: Int = 3; }
// this.callMe(10, 20);
// while (true) { break; }
contract C {
    init() { }
    receive("hello") { }
    fun f(a: Int, b: Int): Int {
        return a + b;
    }
}
"#;

    // NOTE: Keep this list in sync with source above.
    // Lines are 0-based and refer to fully-commented lines only.
    let commented_lines: &[usize] = &[1, 2, 3, 4, 5];

    let fixture = tact_target(source);
    let engine = TactLanguageEngine::new();
    let mutants = engine.mutate(fixture.target());

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
    let cr_nested = mutants.iter().any(|m| m.mutation_slug == "CR" && m.new_text.contains("/* /*"));
    assert!(!cr_nested, "CR should not double-wrap commented content");
}


