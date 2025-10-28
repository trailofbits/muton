use crate::mutations::engine_traits::MutationEngine;
use crate::types::{Mutant, Mutation, MutationSeverity, Target};

use crate::mutations::common::kinds::COMMON_MUTATIONS;
use crate::mutations::common::patterns;
use crate::mutations::common::utils::node_text;
use crate::mutations::parser;
use crate::mutations::tact::kinds::TACT_MUTATIONS;
use crate::mutations::tact::syntax::{fields, nodes};
use crate::types::Language;

pub struct TactMutationEngine {
    mutations: Vec<Mutation>,
}

impl Default for TactMutationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TactMutationEngine {
    pub fn new() -> Self {
        let mut mutations: Vec<Mutation> = Vec::new();
        mutations.extend_from_slice(COMMON_MUTATIONS);
        mutations.extend_from_slice(TACT_MUTATIONS);
        Self { mutations }
    }
}

impl MutationEngine for TactMutationEngine {
    fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn apply_all_mutations(&self, target: &Target) -> Vec<Mutant> {
        let source = &target.text;
        let tree = match parser::parse_for_language(&Language::Tact, source) {
            Some(t) => t,
            None => return Vec::new(),
        };
        let root = tree.root_node();

        let mut all_mutants = Vec::new();
        for m in &self.mutations {
            match m.slug {
                "ER" => {
                    all_mutants.extend(patterns::replace_entire_nodes_of_kinds_filtered(
                        target,
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
                        "ER",
                        "require(false);",
                        &|node, src| {
                            let text = node_text(node, src);
                            // Do not replace statements that already perform an error/require
                            !text.contains("require(")
                        },
                    ));
                }
                "CR" => {
                    all_mutants.extend(patterns::wrap_nodes_of_kinds_with_wrappers(
                        target,
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
                        "CR",
                        "/* ",
                        " */",
                    ));
                }
                "IF" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::IF_STATEMENT,
                    fields::CONDITION,
                    "IF",
                    "false",
                )),
                "IT" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::IF_STATEMENT,
                    fields::CONDITION,
                    "IT",
                    "true",
                )),
                "WF" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::WHILE_STATEMENT,
                    fields::CONDITION,
                    "WF",
                    "false",
                )),
                "RZ" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::REPEAT_STATEMENT,
                    fields::CONDITION,
                    "RZ",
                    "0",
                )),
                "AS" => all_mutants.extend(patterns::swap_adjacent_arguments_for_kinds(
                    target,
                    root,
                    source,
                    &[nodes::METHOD_CALL_EXPRESSION, nodes::STATIC_CALL_EXPRESSION],
                    fields::ARGUMENTS,
                    "AS",
                )),
                // Shared operator shuffles
                "AOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["+", "-", "*", "/"],
                    "AOS",
                )),
                "AAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["+=", "-=", "*=", "/="],
                    "AAOS",
                )),
                "BOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["&", "|", "^"],
                    "BOS",
                )),
                "BAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["&=", "|=", "^="],
                    "BAOS",
                )),
                "UF" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::DO_UNTIL_STATEMENT,
                    fields::CONDITION,
                    "UF",
                    "false",
                )),
                "BL" => all_mutants.extend(patterns::flip_boolean_literals_by_kind(
                    target,
                    root,
                    source,
                    nodes::BOOLEAN,
                    "BL",
                )),
                "COS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["==", "!=", "<", "<=", ">", ">="],
                    "COS",
                )),
                "LOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["&&", "||"],
                    "LOS",
                )),
                "SOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["<<", ">>"],
                    "SOS",
                )),
                "SAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root,
                    source,
                    &[nodes::BINARY_EXPRESSION],
                    &["<<=", ">>="],
                    "SAOS",
                )),
                "TT" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::TERNARY_EXPRESSION,
                    fields::CONDITION,
                    "TT",
                    "true",
                )),
                "TF" => all_mutants.extend(patterns::replace_field_for_nodes_of_kind(
                    target,
                    root,
                    source,
                    nodes::TERNARY_EXPRESSION,
                    fields::CONDITION,
                    "TF",
                    "false",
                )),
                "LC" => all_mutants.extend(patterns::swap_loop_control_statements(
                    target,
                    root,
                    source,
                    nodes::BREAK_STATEMENT,
                    nodes::CONTINUE_STATEMENT,
                    "LC",
                )),
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

    fn get_all_slugs(&self) -> Vec<String> {
        let mut slugs: Vec<String> = self.mutations.iter().map(|m| m.slug.to_string()).collect();
        slugs.sort();
        slugs.dedup();
        slugs
    }

    fn get_severity_by_slug(&self, slug: &str) -> Option<MutationSeverity> {
        self.mutations
            .iter()
            .find(|m| m.slug == slug)
            .map(|m| m.severity.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Hash;
    use std::collections::{BTreeSet, HashSet};
    use std::path::PathBuf;

    #[test]
    fn no_duplicate_slugs_in_combined_mutations() {
        let engine = TactMutationEngine::new();
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
        let text: &str = include_str!("../../..//tests/examples/tact/hello-world.tact");
        let target = Target {
            id: 0,
            path: PathBuf::from("tests/examples/tact/hello-world.tact"),
            file_hash: Hash::digest(text.to_string()),
            text: text.to_string(),
            language: Language::Tact,
        };
        let engine = TactMutationEngine::new();
        let _ = engine.apply_all_mutations(&target);
    }
}
