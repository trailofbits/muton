use muton::parser;
use muton::types::Language;

fn parse_tact(source: &str) -> tree_sitter::Tree {
	let lang = Language::Tact;
	let tree = parser::parse_for_language(&lang, source)
		.expect("Tact parser returned None");
	tree
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
	let mut kinds = std::collections::HashSet::new();
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
fn maps_node_kinds_to_common() {
	use muton::parser::common::node_types::CommonNodeType;
	use muton::parser::tact::map_node_kind_to_common;

	let cases = vec![
		("if_statement", CommonNodeType::IfStatement),
		("while_statement", CommonNodeType::WhileStatement),
		("return_statement", CommonNodeType::ReturnStatement),
		("let_statement", CommonNodeType::LetStatement),
		("binary_expression", CommonNodeType::BinaryExpression),
		("method_call_expression", CommonNodeType::MethodCallExpression),
		("static_call_expression", CommonNodeType::StaticCallExpression),
		("type_identifier", CommonNodeType::TypeIdentifier),
		("comment", CommonNodeType::Comment),
	];

	for (kind, expected) in cases {
		let got = map_node_kind_to_common(kind);
		assert_eq!(got, expected, "mapping for kind '{kind}'");
	}
} 
