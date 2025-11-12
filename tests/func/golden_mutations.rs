use mewt::mutations::func::ast_engine::ASTMutationEngine;
use mewt::types::{Language as MutationLanguage, Target};

fn create_func_target(source: &str) -> Target {
	use tempfile::tempdir;
	let tmp = tempdir().expect("tmpdir");
	let path = tmp.path().join("test.fc");
	std::fs::write(&path, source).unwrap();
	Target {
		id: 1,
		path,
		file_hash: mewt::types::Hash::digest(source.to_string()),
		text: source.to_string(),
		language: MutationLanguage::FunC,
	}
}

fn apply_first_mutant_with_slug(source: &str, slug: &str) -> Option<String> {
	let target = create_func_target(source);
	let engine = ASTMutationEngine::new(&MutationLanguage::FunC);
	let mut mutants: Vec<_> = engine.apply_all_mutations(&target)
		.into_iter().filter(|m| m.mutation_slug == slug).collect();
	mutants.sort_by_key(|m| m.byte_offset);
	mutants.into_iter().next().and_then(|m| target.mutate(&m).ok())
}

#[test]
fn er_replaces_statement_with_throw1() {
	let source = r#"
	() test_func() {
		var x = 42;
	}
	"#;
	let mutated = apply_first_mutant_with_slug(source, "ER").expect("ER mutant");
	let expected = r#"
	() test_func() {
		throw(1);
	}
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn cr_comments_statement_with_double_semicolon() {
	let source = r#"
	() test_func() {
		var x = 42;
	}
	"#;
	let mutated = apply_first_mutant_with_slug(source, "CR").expect("CR mutant");
	let expected = r#"
	() test_func() {
		;; var x = 42;
	}
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn if_condition_true_false() {
	let source = r#"
	() test_func() {
		if (x > 0) {
		}
	}
	"#;
	let m_true = apply_first_mutant_with_slug(source, "IT").expect("IT mutant");
	let m_false = apply_first_mutant_with_slug(source, "IF").expect("IF mutant");
	let expected_true = r#"
	() test_func() {
		if (true) {
		}
	}
	"#;
	let expected_false = r#"
	() test_func() {
		if (false) {
		}
	}
	"#;
	assert_eq!(m_true, expected_true);
	assert_eq!(m_false, expected_false);
}

#[test]
fn while_condition_false_and_repeat_zero() {
	let source_w = r#"
	() test_func() {
		while (x > 0) {
		}
	}
	"#;
	let mw = apply_first_mutant_with_slug(source_w, "WF").expect("WF mutant");
	let expected_w = r#"
	() test_func() {
		while (false) {
		}
	}
	"#;
	assert_eq!(mw, expected_w);

	let source_r = r#"
	() test_func() {
		repeat(10) {
		}
	}
	"#;
	let mr = apply_first_mutant_with_slug(source_r, "RZ").expect("RZ mutant");
	let expected_r = r#"
	() test_func() {
		repeat(0) {
		}
	}
	"#;
	assert_eq!(mr, expected_r);
}

#[test]
fn argument_swap_exact() {
	let source = r#"
	() test_func(int a, int b) {
		foo(a, b);
	}
	"#;
	let mutated = apply_first_mutant_with_slug(source, "AS").expect("AS mutant");
	let expected = r#"
	() test_func(int a, int b) {
		foo(b, a);
	}
	"#;
	assert_eq!(mutated, expected);
} 
