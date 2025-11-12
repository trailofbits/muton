use mewt::types::{Language, Target, Hash};

#[test]
fn end_to_end_generate_mutants_tact() {
	let source = r#"
	contract G {
		fun f(a: Int, b: Int): Int {
			if (a != b) {
				return a;
			}
			return b;
		}
	}
	"#;
	let tmp = tempfile::tempdir().unwrap();
	let path = tmp.path().join("g.tact");
	std::fs::write(&path, source).unwrap();
	let target = Target {
		id: 1,
		path,
		file_hash: Hash::digest(source.to_string()),
		text: source.to_string(),
		language: Language::Tact,
	};

	let mutants = target.generate_mutants().expect("mutants");
	assert!(mutants.len() > 0, "expected at least one mutant");

	// Mutating content should succeed
	let mutated = target.mutate(&mutants[0]).expect("mutated content");
	assert_ne!(mutated, target.text);
} 
