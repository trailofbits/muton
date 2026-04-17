use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use mewt::LanguageEngine;
use muton::languages::func::engine::FuncLanguageEngine;
use muton::languages::tact::engine::TactLanguageEngine;
use muton::languages::tolk::engine::TolkLanguageEngine;

fn engine_slugs(engine: &impl LanguageEngine) -> BTreeSet<String> {
    engine
        .get_mutations()
        .iter()
        .map(|m| m.slug.to_string())
        .collect()
}

fn test_slugs(language: &str) -> BTreeSet<String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(language)
        .join("mutations");
    let mut slugs = BTreeSet::new();

    let entries = fs::read_dir(&dir).unwrap_or_else(|err| {
        panic!("failed to read mutation test directory for {language} at {dir:?}: {err}",)
    });

    for entry in entries {
        let entry = entry.unwrap_or_else(|err| {
            panic!("failed to read entry in {language} mutation directory {dir:?}: {err}",);
        });
        let file_type = entry.file_type().unwrap_or_else(|err| {
            panic!("failed to read file type for {language} mutation entry {dir:?}: {err}",);
        });
        if !file_type.is_file() {
            continue;
        }
        let file_name = entry.file_name().into_string().unwrap_or_else(|name| {
            panic!(
                "non-UTF-8 filename encountered in {language} mutation directory: {:?}",
                name
            );
        });
        if file_name == "mod.rs" {
            continue;
        }
        if !file_name.ends_with(".rs") {
            panic!(
                "unexpected non-Rust file in {language} mutation directory: {file_name} (expected only <SLUG>.rs modules and mod.rs)",
            );
        }
        slugs.insert(file_name.trim_end_matches(".rs").to_string());
    }

    assert!(
        !slugs.is_empty(),
        "no mutation test files discovered for {language} in {dir:?}"
    );

    slugs
}

fn assert_slug_parity(
    language: &str,
    engine_slugs: BTreeSet<String>,
    test_slugs: BTreeSet<String>,
) {
    let missing: Vec<_> = engine_slugs.difference(&test_slugs).cloned().collect();
    let extra: Vec<_> = test_slugs.difference(&engine_slugs).cloned().collect();

    if !missing.is_empty() || !extra.is_empty() {
        panic!(
            "mutation slug module guard failed for {language}\nmissing test modules for engine slugs: {missing:?}\nextra test modules without engine slugs: {extra:?}"
        );
    }
}

#[test]
fn func_mutations_have_tests() {
    let engine = FuncLanguageEngine::new();
    assert_slug_parity("func", engine_slugs(&engine), test_slugs("func"));
}

#[test]
fn tact_mutations_have_tests() {
    let engine = TactLanguageEngine::new();
    assert_slug_parity("tact", engine_slugs(&engine), test_slugs("tact"));
}

#[test]
fn tolk_mutations_have_tests() {
    let engine = TolkLanguageEngine::new();
    assert_slug_parity("tolk", engine_slugs(&engine), test_slugs("tolk"));
}
