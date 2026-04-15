use mewt::LanguageRegistry;
use muton::languages::tact::engine::TactLanguageEngine;

use super::common::tact_target;

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
	let fixture = tact_target(source);
	let target = fixture.target();

	// Create a language registry with the Tact engine
	let mut registry = LanguageRegistry::new();
	registry.register(TactLanguageEngine::new());

	let mutants = target.generate_mutants(&registry).expect("mutants");
	assert!(mutants.len() > 0, "expected at least one mutant");

	// Mutating content should succeed
	let mutated = target.mutate(&mutants[0]).expect("mutated content");
	assert_ne!(mutated, target.text);
} 
