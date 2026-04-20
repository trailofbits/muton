use std::sync::OnceLock;

use mewt::LanguageEngine;
use mewt::mutations::COMMON_MUTATIONS;
use mewt::patterns;
use mewt::types::{Mutant, Mutation, Target};
use mewt::utils::{node_text, parse_source};
use tree_sitter::Language as TsLanguage;

use crate::languages::tolk::kinds::TOLK_MUTATIONS;
use crate::languages::tolk::syntax::{fields, nodes};

static TOLK_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();

unsafe extern "C" {
    fn tree_sitter_tolk() -> *const tree_sitter::ffi::TSLanguage;
}

pub struct TolkLanguageEngine {
    mutations: Vec<Mutation>,
}

impl Default for TolkLanguageEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TolkLanguageEngine {
    pub fn new() -> Self {
        let mut mutations: Vec<Mutation> = Vec::new();
        mutations.extend_from_slice(COMMON_MUTATIONS);
        mutations.extend_from_slice(TOLK_MUTATIONS);
        Self { mutations }
    }
}

impl LanguageEngine for TolkLanguageEngine {
    fn name(&self) -> &'static str {
        "Tolk"
    }

    fn extensions(&self) -> &[&'static str] {
        &["tolk"]
    }

    fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn mutate(&self, target: &Target) -> Vec<Mutant> {
        let source = &target.text;
        let language =
            TOLK_LANGUAGE.get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_tolk()) });
        let tree = match parse_source(source, language) {
            Some(t) => t,
            None => return Vec::new(),
        };
        let root = tree.root_node();

        let mut all_mutants = Vec::new();
        for m in &self.mutations {
            match m.slug {
                "ER" => {
                    all_mutants.extend(
                        patterns::replace(
                            root,
                            source,
                            &[
                                nodes::EXPRESSION_STATEMENT,
                                nodes::RETURN_STATEMENT,
                                nodes::IF_STATEMENT,
                                nodes::WHILE_STATEMENT,
                                nodes::DO_WHILE_STATEMENT,
                            ],
                            "throw 65535;",
                            &|node, src| {
                                // Skip existing error/throw statements
                                !node_text(node, src).contains("throw ")
                            },
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "ER")),
                    );
                }
                "CR" => {
                    all_mutants.extend(
                        patterns::wrap(
                            root,
                            source,
                            &[
                                nodes::EXPRESSION_STATEMENT,
                                nodes::RETURN_STATEMENT,
                                nodes::IF_STATEMENT,
                                nodes::WHILE_STATEMENT,
                                nodes::DO_WHILE_STATEMENT,
                            ],
                            "/* ",
                            " */",
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "CR")),
                    );
                }
                "IF" => {
                    all_mutants.extend(
                        patterns::replace_condition(
                            root,
                            source,
                            nodes::IF_STATEMENT,
                            fields::CONDITION,
                            &["if"],
                            "false",
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "IF")),
                    );
                }
                "IT" => {
                    all_mutants.extend(
                        patterns::replace_condition(
                            root,
                            source,
                            nodes::IF_STATEMENT,
                            fields::CONDITION,
                            &["if"],
                            "true",
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "IT")),
                    );
                }
                "WF" => {
                    all_mutants.extend(
                        patterns::replace_condition(
                            root,
                            source,
                            nodes::WHILE_STATEMENT,
                            fields::CONDITION,
                            &["while"],
                            "false",
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "WF")),
                    );
                }
                "AS" => {
                    all_mutants.extend(
                        patterns::swap_args(
                            root,
                            source,
                            &[nodes::FUNCTION_CALL],
                            fields::ARGUMENTS,
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "AS")),
                    );
                }
                "LC" => all_mutants.extend(
                    patterns::shuffle_nodes(
                        root,
                        source,
                        &[nodes::BREAK_STATEMENT, nodes::CONTINUE_STATEMENT],
                        &["break", "continue"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "LC")),
                ),
                "BL" => {
                    all_mutants.extend(
                        patterns::shuffle_nodes(
                            root,
                            source,
                            &[nodes::BOOLEAN_LITERAL],
                            &["true", "false"],
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "BL")),
                    );
                }

                // Shared operator shuffles via common patterns
                "AOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_OPERATOR],
                        &["+", "-", "*", "/"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "AOS")),
                ),
                "AAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::ASSIGNMENT, nodes::SET_ASSIGNMENT],
                        &["+=", "-=", "*=", "/="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "AAOS")),
                ),
                "BOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_OPERATOR],
                        &["&", "|", "^"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "BOS")),
                ),
                "BAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::ASSIGNMENT, nodes::SET_ASSIGNMENT],
                        &["&=", "|=", "^="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "BAOS")),
                ),
                "COS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_OPERATOR],
                        &["==", "!=", "<", "<=", ">", ">="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "COS")),
                ),
                "LOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_OPERATOR],
                        &["&&", "||"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "LOS")),
                ),
                "SOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_OPERATOR],
                        &["<<", ">>"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "SOS")),
                ),
                "SAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::ASSIGNMENT, nodes::SET_ASSIGNMENT],
                        &["<<=", ">>="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "SAOS")),
                ),
                "NR" => all_mutants.extend(
                    patterns::remove_unary_operator(
                        root,
                        source,
                        nodes::UNARY_OPERATOR,
                        fields::OPERATOR_NAME,
                        fields::ARGUMENT,
                        "!",
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "NR")),
                ),
                _ => {
                    panic!(
                        "Unknown mutation slug encountered in Tolk engine: {}",
                        m.slug
                    );
                }
            }
        }

        all_mutants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashSet};
    use std::path::PathBuf;

    #[test]
    fn no_duplicate_slugs_in_combined_mutations() {
        let engine = TolkLanguageEngine::new();
        let mut seen: HashSet<&str> = HashSet::new();
        let mut dups: BTreeSet<String> = BTreeSet::new();
        for m in engine.get_mutations() {
            if !seen.insert(m.slug) {
                dups.insert(m.slug.to_string());
            }
        }
        assert!(
            dups.is_empty(),
            "Duplicate mutation slugs found in Tolk engine: {dups:?}",
        );
    }

    #[test]
    fn all_defined_slugs_have_match_arms() {
        // Use a simple Tolk program for smoke testing
        let text: &str = "fun main() { return; }";
        let target = Target {
            id: 0,
            path: PathBuf::from("test.tolk"),
            file_hash: mewt::types::Hash::digest(text.to_string()),
            text: text.to_string(),
            language: "Tolk".to_string(),
        };
        let engine = TolkLanguageEngine::new();
        // Will panic if any slug is missing a match arm (default case)
        let _ = engine.mutate(&target);
    }
}
