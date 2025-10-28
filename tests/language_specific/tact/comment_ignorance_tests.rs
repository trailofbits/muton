use muton::{mutations, types::{Language, Target, Hash}};

fn tact_target_from_source(source: &str) -> Target {
    use tempfile::tempdir;
    let tmp = tempdir().expect("tmpdir");
    let path = tmp.path().join("test.tact");
    std::fs::write(&path, source).unwrap();
    Target {
        id: 1,
        path,
        file_hash: Hash::digest(source.to_string()),
        text: source.to_string(),
        language: Language::Tact,
    }
}

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

    let target = tact_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::Tact);
    let mutants = engine.apply_all_mutations(&target);

    // Ensure none of the mutants originate from commented content (line or block)
    fn block_spans(src: &str, open: &str, close: &str) -> Vec<(usize, usize)> {
        let mut spans = Vec::new();
        let mut i = 0;
        while let Some(open_rel) = src[i..].find(open) {
            let start = i + open_rel;
            if let Some(close_rel) = src[start + open.len()..].find(close) {
                let end = start + open.len() + close_rel + close.len();
                spans.push((start, end));
                i = end;
            } else {
                break;
            }
        }
        spans
    }

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


