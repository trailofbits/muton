use crate::types::{Language, MutationSeverity};

pub mod common;
pub mod engine_traits;
pub mod func;
pub mod parser;
pub mod tact;

/// Get mutations for a specific language
pub fn get_mutations_for_language(language: &Language) -> Box<dyn engine_traits::MutationEngine> {
    match language {
        Language::FunC => Box::new(func::engine::FuncMutationEngine::new()),
        Language::Tact => Box::new(tact::engine::TactMutationEngine::new()),
    }
}

/// Get the severity for a mutation slug
pub fn get_severity_by_slug(slug: &str, language: &Language) -> Option<MutationSeverity> {
    let engine = get_mutations_for_language(language);
    engine.get_severity_by_slug(slug)
}

/// Get all unique mutation slugs
pub fn get_all_slugs(language: &Language) -> Vec<String> {
    let engine = get_mutations_for_language(language);
    engine.get_all_slugs()
}
