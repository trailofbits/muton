#[path = "conformance.rs"]
mod conformance;
#[path = "utils.rs"]
mod utils;

mod func;
mod tact;
mod tolk;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use mewt::LanguageEngine;
use muton::languages::func::engine::FuncLanguageEngine;
use muton::languages::tact::engine::TactLanguageEngine;
use muton::languages::tolk::engine::TolkLanguageEngine;

#[test]
fn every_mutation_slug_has_a_per_language_test_module() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let func = FuncLanguageEngine::new();
    check_language(manifest_dir, "FunC", "func", &func);

    let tact = TactLanguageEngine::new();
    check_language(manifest_dir, "Tact", "tact", &tact);

    let tolk = TolkLanguageEngine::new();
    check_language(manifest_dir, "Tolk", "tolk", &tolk);
}

fn check_language(
    manifest_dir: &Path,
    language_name: &str,
    language_dir: &str,
    engine: &dyn LanguageEngine,
) {
    let slug_set: BTreeSet<String> = engine
        .get_mutations()
        .iter()
        .map(|m| m.slug.to_string())
        .collect();

    let modules_dir = manifest_dir
        .join("tests")
        .join(language_dir)
        .join("mutations");
    assert!(
        modules_dir.is_dir(),
        "expected mutation test directory for {language_name} at {modules_dir:?}"
    );

    let file_slugs: BTreeSet<String> = fs::read_dir(&modules_dir)
        .unwrap_or_else(|err| {
            panic!(
                "failed to read mutation test directory for {language_name} at {modules_dir:?}: {err}"
            )
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                return None;
            }
            let stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())?;
            if stem == "mod" {
                return None;
            }
            Some(stem)
        })
        .collect();

    let missing: Vec<String> = slug_set.difference(&file_slugs).cloned().collect();
    let unexpected: Vec<String> = file_slugs.difference(&slug_set).cloned().collect();

    assert!(
        missing.is_empty(),
        "{language_name} is missing mutation test modules for slugs: {missing:?}"
    );
    assert!(
        unexpected.is_empty(),
        "{language_name} has mutation test modules without corresponding slugs: {unexpected:?}"
    );
}
