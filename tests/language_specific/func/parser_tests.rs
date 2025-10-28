use muton::parser;
use muton::types::Language;

fn parse_func(source: &str) -> tree_sitter::Tree {
	let lang = Language::FunC;
	let tree = parser::parse_for_language(&lang, source)
		.expect("FunC parser returned None");
	tree
}

#[test]
fn parses_minimal_function() {
	let source = r#"() test() { return 1; }"#;
	let tree = parse_func(source);
	let root = tree.root_node();
	assert_eq!(root.kind(), "source_file");
	// Ensure we have at least one named child
	let mut cursor = root.walk();
	assert!(root.named_children(&mut cursor).count() > 0);
}

#[test]
fn parses_hello_world_example_file() {
	let path = "tests/examples/func/hello-world.fc";
	let source = std::fs::read_to_string(path).expect("failed to read hello-world.fc example");
	let tree = parse_func(&source);
	let root = tree.root_node();
	assert_eq!(root.kind(), "source_file");
	// Heuristic sanity: expect multiple named top-level nodes
	let mut cursor = root.walk();
	let top_named: usize = root.named_children(&mut cursor).count();
	assert!(top_named >= 2, "expected multiple top-level items in example");
} 