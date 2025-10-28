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

fn apply_first_mutant_with_slug(source: &str, slug: &str) -> Option<String> {
	let target = tact_target_from_source(source);
	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let mut mutants: Vec<_> = engine.apply_all_mutations(&target)
		.into_iter().filter(|m| m.mutation_slug == slug).collect();
	mutants.sort_by_key(|m| m.byte_offset);
	mutants.into_iter().next().and_then(|m| target.mutate(&m).ok())
}

#[test]
fn er_replaces_single_statement_exact() {
	let source = r#"
	contract C { fun f() { let x: Int = 1; } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "ER").expect("ER mutant");
	let expected = r#"
	contract C { fun f() { require(false); } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn cr_wraps_statement_in_block_comment() {
	let source = r#"
	contract C { fun f() { let x: Int = 1; } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "CR").expect("CR mutant");
	let expected = r#"
	contract C { fun f() { /* let x: Int = 1; */ } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn if_condition_hardcoded_false_true() {
	let source = r#"
	contract C { fun f(a: Int, b: Int) { if (a > b) { } } }
	"#;
	let it = apply_first_mutant_with_slug(source, "IT").expect("IT mutant");
	let ifalse = apply_first_mutant_with_slug(source, "IF").expect("IF mutant");
	let expected_true = r#"
	contract C { fun f(a: Int, b: Int) { if (true) { } } }
	"#;
	let expected_false = r#"
	contract C { fun f(a: Int, b: Int) { if (false) { } } }
	"#;
	assert_eq!(it, expected_true);
	assert_eq!(ifalse, expected_false);
}

#[test]
fn while_condition_hardcoded_false() {
	let source = r#"
	contract C { fun f(a: Int) { while (a > 0) { } } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "WF").expect("WF mutant");
	let expected = r#"
	contract C { fun f(a: Int) { while (false) { } } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn argument_swap_exact() {
	let source = r#"
	contract D { fun f(a: Int, b: Int) { this.doIt(a, b); } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "AS").expect("AS mutant");
	let expected = r#"
	contract D { fun f(a: Int, b: Int) { this.doIt(b, a); } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn boolean_flip_exact() {
	let source = r#"
	contract E { fun f() { if (true) { } } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "BL").expect("BL mutant");
	let expected = r#"
	contract E { fun f() { if (false) { } } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn ternary_condition_true_false_exact() {
	let source = r#"
	contract F { fun f(a: Int): Int { let v: Int = if (a > 0) 1 else 2; return v; } }
	"#;
	let tt = apply_first_mutant_with_slug(source, "TT").expect("TT mutant");
	let tf = apply_first_mutant_with_slug(source, "TF").expect("TF mutant");
	let expected_t = r#"
	contract F { fun f(a: Int): Int { let v: Int = if (true) 1 else 2; return v; } }
	"#;
	let expected_f = r#"
	contract F { fun f(a: Int): Int { let v: Int = if (false) 1 else 2; return v; } }
	"#;
	assert_eq!(tt, expected_t);
	assert_eq!(tf, expected_f);
}

#[test]
fn do_until_condition_false_exact() {
	let source = r#"
	contract G { fun f(a: Int) { do { } until (a == 0); } }
	"#;
	let mutated = apply_first_mutant_with_slug(source, "UF").expect("UF mutant");
	let expected = r#"
	contract G { fun f(a: Int) { do { } until (false); } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn comparison_shuffle_includes_not_equal() {
	let source = r#"
	contract H { fun f(x: Int, y: Int) { if (x == y) { } } }
	"#;
	// pick a COS mutant that turns == into != specifically
	let target = tact_target_from_source(source);
	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let cos = engine.apply_all_mutations(&target).into_iter().find(|m| m.mutation_slug == "COS" && m.old_text == "==" && m.new_text == "!=");
	let mutated = cos.and_then(|m| target.mutate(&m).ok()).expect("COS ==->!= mutant");
	let expected = r#"
	contract H { fun f(x: Int, y: Int) { if (x != y) { } } }
	"#;
	assert_eq!(mutated, expected);
}

#[test]
fn generates_basic_tact_mutations() {
	let source = r#"
	contract C {
		fun f(a: Int, b: Int): Int {
			let x: Int = 1;
			if (a > b) { return a; }
			while (a < b) { return b; }
			return 0;
		}
	}
	"#;
	let target = tact_target_from_source(source);

	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let mutants = engine.apply_all_mutations(&target);
	assert!(!mutants.is_empty(), "expected some mutants for Tact source");

	let slugs: std::collections::HashSet<_> = mutants.iter().map(|m| m.mutation_slug.as_str()).collect();
	// Expect a subset of core slugs to appear
	assert!(slugs.contains("ER"));
	assert!(slugs.contains("CR"));
	assert!(slugs.contains("IF"));
	assert!(slugs.contains("WF"));
}

#[test]
fn argument_swap_on_method_and_static_calls() {
	let source = r#"
	contract D {
		fun f(): Int {
			let a: Int = 1;
			let b: Int = 2;
			// method call
			this.doIt(a, b);
			// static call
			Math::mul(a, b);
			return 0;
		}
	}
	"#;
	let target = tact_target_from_source(source);
	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let mutants = engine.apply_all_mutations(&target);

	let as_mutants: Vec<_> = mutants.iter().filter(|m| m.mutation_slug == "AS").collect();
	assert!(!as_mutants.is_empty(), "expected AS mutations");
	// Ensure swapped text contains "b, a" at least once
	assert!(as_mutants.iter().any(|m| m.new_text.contains("b, a")));
}

#[test]
fn boolean_flip_and_comparison_shuffle() {
	let source = r#"
	contract E {
		fun f(x: Int, y: Int): Int {
			if (true) { return 1; }
			if (x == y) { return 2; }
			return 0;
		}
	}
	"#;
	let target = tact_target_from_source(source);
	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let mutants = engine.apply_all_mutations(&target);

	let bl = mutants.iter().any(|m| m.mutation_slug == "BL" && (m.old_text == "true" || m.old_text == "false"));
	assert!(bl, "expected BL boolean flip mutants");

	let cos = mutants.iter().any(|m| m.mutation_slug == "COS");
	assert!(cos, "expected COS comparison operator shuffle mutants");
}

#[test]
fn ternary_and_do_until_mutations() {
	let source = r#"
	contract F {
		fun f(a: Int): Int {
			let v: Int = if (a > 0) 1 else 2;
			do { } until (a == 0);
			return v;
		}
	}
	"#;
	let target = tact_target_from_source(source);
	let engine = mutations::get_mutations_for_language(&Language::Tact);
	let mutants = engine.apply_all_mutations(&target);

	let tt = mutants.iter().any(|m| m.mutation_slug == "TT");
	let tf = mutants.iter().any(|m| m.mutation_slug == "TF");
	let uf = mutants.iter().any(|m| m.mutation_slug == "UF");
	assert!(tt && tf && uf, "expected TT, TF, and UF mutants");
} 
