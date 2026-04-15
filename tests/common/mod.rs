use std::collections::{HashMap, HashSet};
use std::path::Path;

use mewt::types::{Hash, Language, Mutant, Target};
use tempfile::TempDir;

/// Keeps the temporary directory alive for the lifetime of a test target and
/// exposes convenient accessors for the embedded [`Target`].
#[derive(Debug)]
pub struct TargetFixture {
    temp_dir: TempDir,
    target: Target,
}

impl TargetFixture {
    /// Create a new [`TargetFixture`] for the given language, file extension, and
    /// source snippet.
    pub fn new(language: impl Into<Language>, extension: &str, source: &str) -> Self {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir for test target");
        let file_name = format!("test.{extension}");
        let path = temp_dir.path().join(file_name);
        std::fs::write(&path, source).expect("failed to write test source");

        let text = source.to_string();
        let target = Target {
            id: 1,
            path,
            file_hash: Hash::digest(text.clone()),
            text,
            language: language.into(),
        };

        Self { temp_dir, target }
    }

    /// Borrow the underlying [`Target`].
    pub fn target(&self) -> &Target {
        &self.target
    }

    /// Consume the fixture, returning the owned [`Target`].
    pub fn into_target(self) -> Target {
        self.target
    }

    /// Borrow the [`TempDir`] keeping the backing file alive.
    pub fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }

    /// Return the on-disk path for the target source file.
    pub fn path(&self) -> &Path {
        self.target.path.as_path()
    }

    /// Borrow the original source text.
    pub fn text(&self) -> &str {
        &self.target.text
    }
}

/// Create a FunC test target.
pub fn func_target(source: &str) -> TargetFixture {
    TargetFixture::new(Language::FunC, "fc", source)
}

/// Create a Tact test target.
pub fn tact_target(source: &str) -> TargetFixture {
    TargetFixture::new(Language::Tact, "tact", source)
}

/// Create a Tolk test target.
pub fn tolk_target(source: &str) -> TargetFixture {
    TargetFixture::new("Tolk", "tolk", source)
}

/// Count how many mutants share the same slug.
pub fn slug_counts(mutants: &[Mutant]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for mutant in mutants {
        *counts.entry(mutant.mutation_slug.clone()).or_default() += 1;
    }
    counts
}

/// Return the first mutant (ordered by byte offset) with the requested slug, if any.
pub fn first_mutant_with_slug<'a>(mutants: &'a [Mutant], slug: &str) -> Option<&'a Mutant> {
    mutants
        .iter()
        .filter(|m| m.mutation_slug == slug)
        .min_by_key(|m| m.byte_offset)
}

/// Collect the distinct mutation slugs present in the provided mutants.
pub fn slug_set(mutants: &[Mutant]) -> HashSet<String> {
    mutants.iter().map(|m| m.mutation_slug.clone()).collect()
}

/// Sort a mutant vector in-place by byte offset for deterministic assertions.
pub fn sort_by_byte_offset(mutants: &mut [Mutant]) {
    mutants.sort_by_key(|m| m.byte_offset);
}
