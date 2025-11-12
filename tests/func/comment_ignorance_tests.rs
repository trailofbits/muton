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

    let target = func_target_from_source(source);
    let engine = mutations::get_mutations_for_language(&Language::FunC);
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
    let cr_nested = mutants.iter().any(|m| m.mutation_slug == "CR" && m.new_text.contains("{- {-"));
    assert!(!cr_nested, "CR should not double-wrap commented content");
}


