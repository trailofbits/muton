use crate::types::{Mutant, Mutation, MutationSeverity, Target};

/// Language-agnostic mutation engine interface
pub trait MutationEngine {
    fn get_mutations(&self) -> &[Mutation];
    fn apply_all_mutations(&self, target: &Target) -> Vec<Mutant>;
    fn get_all_slugs(&self) -> Vec<String>;
    fn get_severity_by_slug(&self, slug: &str) -> Option<MutationSeverity>;
}
