use std::collections::HashSet;
use std::sync::OnceLock;

use mewt::utils::parse_source;
use tree_sitter::Language as TsLanguage;

static TACT_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();

unsafe extern "C" {
	fn tree_sitter_tact() -> *const tree_sitter::ffi::TSLanguage;
}

fn tact_language() -> &'static TsLanguage {
	TACT_LANGUAGE.get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_tact()) })
}

fn parse_tact(source: &str) -> tree_sitter::Tree {
	parse_source(source, tact_language()).expect("Tact parser returned None")
}

#[test]
fn parses_minimal_contract() {
	let source = r#"
	contract Hello {
	}
	"#;
	let tree = parse_tact(source);
	let root = tree.root_node();
	assert_eq!(root.kind(), "source_file");
	// Ensure we have at least one child (contract)
	let mut cursor = root.walk();
	let mut found_contract = false;
	for child in root.named_children(&mut cursor) {
		if child.kind() == "contract" {
			found_contract = true;
			break;
		}
	}
	assert!(found_contract, "expected to find a contract node");
}

#[test]
fn parses_functions_and_statements() {
	let source = r#"
	fun add(a: Int, b: Int): Int { return a + b; }
	fun noop(): Int; // declaration without body
	contract Math {
		let counter: Int = 0;
		init() { let x: Int = 1; }
		receive("msg") { let y: Int = 2; if (y > 0) { return; } }
		fun mul(a: Int, b: Int): Int { return a * b; }
	}
	"#;
	let tree = parse_tact(source);
	let root = tree.root_node();
	let mut cursor = root.walk();
	let mut kinds = HashSet::new();
	for child in root.named_children(&mut cursor) {
		kinds.insert(child.kind().to_string());
	}
	// Sanity checks for presence of high-level constructs
	assert!(kinds.contains("global_function") || kinds.contains("storage_function"));
	assert!(kinds.contains("contract"));
}

#[test]
fn contains_core_statement_kinds_inside_contract() {
	let source = r#"
	contract Flow {
		fun test(a: Int): Int {
			let x: Int = a;
			if (x > 0) { return x; }
			while (x > 0) { x = x - 1; }
			return 0;
		}
	}
	"#;
	let tree = parse_tact(source);
	let root = tree.root_node();
	let mut cursor = root.walk();
	let mut saw_if = false;
	let mut saw_while = false;
	let mut saw_return = false;
	let mut saw_let = false;
	let mut stack = vec![root];
	while let Some(node) = stack.pop() {
		let mut c = node.walk();
		for ch in node.named_children(&mut c) {
			match ch.kind() {
				"if_statement" => saw_if = true,
				"while_statement" => saw_while = true,
				"return_statement" => saw_return = true,
				"let_statement" => saw_let = true,
				_ => {}
			}
			stack.push(ch);
		}
	}
	assert!(saw_if, "expected to find if_statement");
	assert!(saw_while, "expected to find while_statement");
	assert!(saw_return, "expected to find return_statement");
	assert!(saw_let, "expected to find let_statement");
}

#[test]
fn finds_comment_nodes() {
	let source = r#"
	contract Annotated {
		fun f(a: Int): Int {
			/* block comment */
			let x: Int = 1; // line comment
			return x;
		}
	}
	"#;
	let tree = parse_tact(source);
	let root = tree.root_node();
	let mut saw_block = false;
	let mut saw_line = false;
	let mut stack = vec![root];
	while let Some(node) = stack.pop() {
		let mut cursor = node.walk();
		for child in node.named_children(&mut cursor) {
			match child.kind() {
				"block_comment" => saw_block = true,
				"line_comment" => saw_line = true,
				_ => {}
			}
			stack.push(child);
		}
	}
	assert!(saw_block, "expected to find block_comment node");
	assert!(saw_line, "expected to find line_comment node");
}
