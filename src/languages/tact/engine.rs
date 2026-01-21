use std::sync::OnceLock;

use mewt::LanguageEngine;
use mewt::mutations::COMMON_MUTATIONS;
use mewt::patterns;
use mewt::types::{Mutant, Mutation, Target};
use mewt::utils::node_text;
use tree_sitter::Language as TsLanguage;

use crate::languages::tact::kinds::TACT_MUTATIONS;
use crate::languages::tact::syntax::{fields, nodes};

static TACT_LANGUAGE: OnceLock<TsLanguage> = OnceLock::new();

unsafe extern "C" {
    fn tree_sitter_tact() -> *const tree_sitter::ffi::TSLanguage;
}

pub struct TactLanguageEngine {
    mutations: Vec<Mutation>,
}

impl Default for TactLanguageEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TactLanguageEngine {
    pub fn new() -> Self {
        let mut mutations: Vec<Mutation> = Vec::new();
        mutations.extend_from_slice(COMMON_MUTATIONS);
        mutations.extend_from_slice(TACT_MUTATIONS);
        Self { mutations }
    }

    fn parse(&self, source: &str) -> Option<tree_sitter::Tree> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&self.tree_sitter_language()).ok()?;
        parser.parse(source, None)
    }
}

impl LanguageEngine for TactLanguageEngine {
    fn name(&self) -> &'static str {
        "Tact"
    }

    fn extensions(&self) -> &[&'static str] {
        &["tact"]
    }

    fn tree_sitter_language(&self) -> TsLanguage {
        TACT_LANGUAGE
            .get_or_init(|| unsafe { TsLanguage::from_raw(tree_sitter_tact()) })
            .clone()
    }

    fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn apply_all_mutations(&self, target: &Target) -> Vec<Mutant> {
        let source = &target.text;
        let tree = match self.parse(source) {
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
                                nodes::LET_STATEMENT,
                                nodes::DESTRUCT_STATEMENT,
                                nodes::IF_STATEMENT,
                                nodes::WHILE_STATEMENT,
                                nodes::DO_UNTIL_STATEMENT,
                                nodes::REPEAT_STATEMENT,
                                nodes::FOREACH_STATEMENT,
                            ],
                            "require(false);",
                            &|node, src| {
                                let text = node_text(node, src);
                                // Do not replace statements that already perform an error/require
                                !text.contains("require(")
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
                                nodes::LET_STATEMENT,
                                nodes::DESTRUCT_STATEMENT,
                                nodes::IF_STATEMENT,
                                nodes::WHILE_STATEMENT,
                                nodes::DO_UNTIL_STATEMENT,
                                nodes::REPEAT_STATEMENT,
                                nodes::FOREACH_STATEMENT,
                            ],
                            "/* ",
                            " */",
                        )
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "CR")),
                    );
                }
                "IF" => all_mutants.extend(
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
                ),
                "IT" => all_mutants.extend(
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
                ),
                "WF" => all_mutants.extend(
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
                ),
                "RZ" => all_mutants.extend(
                    patterns::replace_condition(
                        root,
                        source,
                        nodes::REPEAT_STATEMENT,
                        fields::CONDITION,
                        &["repeat"],
                        "0",
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "RZ")),
                ),
                "AS" => all_mutants.extend(
                    patterns::swap_args(
                        root,
                        source,
                        &[nodes::METHOD_CALL_EXPRESSION, nodes::STATIC_CALL_EXPRESSION],
                        fields::ARGUMENTS,
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "AS")),
                ),
                // Shared operator shuffles
                "AOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["+", "-", "*", "/"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "AOS")),
                ),
                "AAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["+=", "-=", "*=", "/="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "AAOS")),
                ),
                "BOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["&", "|", "^"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "BOS")),
                ),
                "BAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["&=", "|=", "^="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "BAOS")),
                ),
                "UF" => all_mutants.extend(
                    patterns::replace_condition(
                        root,
                        source,
                        nodes::DO_UNTIL_STATEMENT,
                        fields::CONDITION,
                        &["until"],
                        "false",
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "UF")),
                ),
                "BL" => all_mutants.extend(
                    patterns::shuffle_nodes(root, source, &[nodes::BOOLEAN], &["true", "false"])
                        .into_iter()
                        .map(|p| Mutant::from_partial(p, target, "BL")),
                ),
                "COS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["==", "!=", "<", "<=", ">", ">="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "COS")),
                ),
                "LOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["&&", "||"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "LOS")),
                ),
                "SOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["<<", ">>"],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "SOS")),
                ),
                "SAOS" => all_mutants.extend(
                    patterns::shuffle_operators(
                        root,
                        source,
                        &[nodes::BINARY_EXPRESSION],
                        &["<<=", ">>="],
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "SAOS")),
                ),
                "TT" => all_mutants.extend(
                    patterns::replace_condition(
                        root,
                        source,
                        nodes::TERNARY_EXPRESSION,
                        fields::CONDITION,
                        &["?"],
                        "true",
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "TT")),
                ),
                "TF" => all_mutants.extend(
                    patterns::replace_condition(
                        root,
                        source,
                        nodes::TERNARY_EXPRESSION,
                        fields::CONDITION,
                        &["?"],
                        "false",
                    )
                    .into_iter()
                    .map(|p| Mutant::from_partial(p, target, "TF")),
                ),
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
                _ => {
                    panic!(
                        "Unknown mutation slug encountered in Tact engine: {}",
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
        let engine = TactLanguageEngine::new();
        let mut seen: HashSet<&str> = HashSet::new();
        let mut dups: BTreeSet<String> = BTreeSet::new();
        for m in engine.get_mutations() {
            if !seen.insert(m.slug) {
                dups.insert(m.slug.to_string());
            }
        }
        assert!(
            dups.is_empty(),
            "Duplicate mutation slugs found in Tact engine: {dups:?}",
        );
    }

    #[test]
    fn all_defined_slugs_have_match_arms() {
        // Use a simple Tact program for smoke testing
        let text: &str = "contract Test { }";
        let target = Target {
            id: 0,
            path: PathBuf::from("test.tact"),
            file_hash: mewt::types::Hash::digest(text.to_string()),
            text: text.to_string(),
            language: "Tact".to_string(),
        };
        let engine = TactLanguageEngine::new();
        let _ = engine.apply_all_mutations(&target);
    }
}
