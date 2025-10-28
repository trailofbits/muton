use std::sync::OnceLock;
use tree_sitter::{Language as TsLanguage, Parser, Tree};

use crate::types::Language;

static FUNC_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();
static TACT_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();

unsafe extern "C" {
    fn tree_sitter_func() -> *const tree_sitter::ffi::TSLanguage;
    fn tree_sitter_tact() -> *const tree_sitter::ffi::TSLanguage;
}

pub fn language_for(language: &Language) -> TsLanguage {
    match language {
        Language::FunC => FUNC_LANGUAGE
            .get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_func()) })
            .clone(),
        Language::Tact => TACT_LANGUAGE
            .get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_tact()) })
            .clone(),
    }
}

pub fn parse_for_language(language: &Language, source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&language_for(language))
        .expect("Error loading grammar");
    parser.parse(source, None)
}
