use std::sync::OnceLock;

use mewt::utils::parse_source;
use tree_sitter::Language as TsLanguage;

static FUNC_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();

unsafe extern "C" {
    fn tree_sitter_func() -> *const tree_sitter::ffi::TSLanguage;
}

fn func_language() -> &'static TsLanguage {
    FUNC_LANGUAGE.get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_func()) })
}

fn parse_func(source: &str) -> tree_sitter::Tree {
    parse_source(source, func_language()).expect("FunC parser returned None")
}

#[test]
fn parses_minimal_function() {
    let source = r#"() test() { return 1; }"#;
    let tree = parse_func(source);
    let root = tree.root_node();
    assert_eq!(root.kind(), "translation_unit");
    // Ensure we have at least one named child
    let mut cursor = root.walk();
    assert!(root.named_children(&mut cursor).count() > 0);
}
