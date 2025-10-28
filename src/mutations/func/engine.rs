use crate::types::{Mutant, MutationSeverity, Target};

use crate::mutations::common::kinds::COMMON_MUTATIONS;
use crate::mutations::common::patterns;
use crate::mutations::common::utils::node_text;
use crate::mutations::engine_traits::MutationEngine;
use crate::mutations::func::kinds::FUNC_MUTATIONS;
use crate::mutations::func::syntax::{fields, nodes};
use crate::mutations::parser;
use crate::types::Language;
use crate::types::Mutation;

pub struct FuncMutationEngine {
    mutations: Vec<Mutation>,
}

impl Default for FuncMutationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FuncMutationEngine {
    pub fn new() -> Self {
        let mut mutations: Vec<Mutation> = Vec::new();
        mutations.extend_from_slice(COMMON_MUTATIONS);
        mutations.extend_from_slice(FUNC_MUTATIONS);
        Self { mutations }
    }

    /// Get all mutations for this engine
    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    /// Apply all mutations to a target
    pub fn apply_all_mutations(&self, target: &Target) -> Vec<Mutant> {
        let source = &target.text;
        let tree = match parser::parse_for_language(&Language::FunC, source) {
            Some(t) => t,
            None => return Vec::new(),
        };
        let root_node = tree.root_node();

        let mut all_mutants = Vec::new();

        for m in &self.mutations {
            match m.slug {
                "ER" => {
                    all_mutants.extend(patterns::replace_entire_nodes_of_kinds_filtered(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::EXPRESSION_STATEMENT,
                            nodes::RETURN_STATEMENT,
                            nodes::ASSIGNMENT_STATEMENT,
                            nodes::VARIABLE_DECLARATION,
                            nodes::IF_STATEMENT,
                        ],
                        "ER",
                        "throw(1);",
                        &|node, src| {
                            let text = node_text(node, src);
                            // Skip existing error/throw statements
                            !text.contains("throw(")
                        },
                    ));
                }
                "CR" => {
                    all_mutants.extend(patterns::wrap_nodes_of_kinds_with_wrappers(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::EXPRESSION_STATEMENT,
                            nodes::RETURN_STATEMENT,
                            nodes::ASSIGNMENT_STATEMENT,
                            nodes::VARIABLE_DECLARATION,
                            nodes::IF_STATEMENT,
                        ],
                        "CR",
                        "{- ",
                        " -}",
                    ));
                }
                "IF" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::IF_STATEMENT,
                        fields::CONDITION,
                        &["if"],
                        "IF",
                        "false",
                    ));
                }
                "IT" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::IF_STATEMENT,
                        fields::CONDITION,
                        &["if"],
                        "IT",
                        "true",
                    ));
                }
                "INF" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::IFNOT_STATEMENT,
                        fields::CONDITION,
                        &["ifnot"],
                        "INF",
                        "false",
                    ));
                }
                "INT" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::IFNOT_STATEMENT,
                        fields::CONDITION,
                        &["ifnot"],
                        "INT",
                        "true",
                    ));
                }
                "WF" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::WHILE_STATEMENT,
                        fields::CONDITION,
                        &["while"],
                        "WF",
                        "false",
                    ));
                }
                "RZ" => {
                    all_mutants.extend(patterns::replace_repeat_count_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::REPEAT_STATEMENT,
                        fields::COUNT,
                        &["repeat"],
                        "RZ",
                        "0",
                    ));
                }
                "UF" => {
                    all_mutants.extend(patterns::replace_condition_for_nodes_of_kind(
                        target,
                        root_node,
                        source,
                        nodes::UNTIL_STATEMENT,
                        fields::CONDITION,
                        &["until"],
                        "UF",
                        "false",
                    ));
                }
                "AS" => {
                    // Retain existing args-field path; fallback is handled by separate helper call below
                    all_mutants.extend(patterns::swap_adjacent_arguments_for_kinds(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::CALL_EXPRESSION,
                            nodes::FUNCTION_APPLICATION,
                            nodes::METHOD_CALL,
                        ],
                        fields::ARGUMENTS,
                        "AS",
                    ));
                    // Fallback: try again where arguments are in an alternate list container
                    all_mutants.extend(patterns::swap_adjacent_arguments_for_kinds(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::CALL_EXPRESSION,
                            nodes::FUNCTION_APPLICATION,
                            nodes::METHOD_CALL,
                        ],
                        nodes::ARGUMENT_LIST,
                        "AS",
                    ));
                }
                "SU" => {
                    all_mutants.extend(patterns::replace_first_argument_for_calls_matching(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::CALL_EXPRESSION,
                            nodes::FUNCTION_APPLICATION,
                            nodes::METHOD_CALL,
                        ],
                        fields::ARGUMENTS,
                        &[nodes::ARGUMENT_LIST],
                        "SU",
                        &|callee: &str| callee.contains("store_uint"),
                        "0",
                    ));
                }
                "SI" => {
                    all_mutants.extend(patterns::replace_first_argument_for_calls_matching(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::CALL_EXPRESSION,
                            nodes::FUNCTION_APPLICATION,
                            nodes::METHOD_CALL,
                        ],
                        fields::ARGUMENTS,
                        &[nodes::ARGUMENT_LIST],
                        "SI",
                        &|callee: &str| callee.contains("store_int"),
                        "0",
                    ));
                }
                "SC" => {
                    all_mutants.extend(patterns::replace_first_argument_for_calls_matching(
                        target,
                        root_node,
                        source,
                        &[
                            nodes::CALL_EXPRESSION,
                            nodes::FUNCTION_APPLICATION,
                            nodes::METHOD_CALL,
                        ],
                        fields::ARGUMENTS,
                        &[nodes::ARGUMENT_LIST],
                        "SC",
                        &|callee: &str| callee.contains("store_coins"),
                        "0",
                    ));
                }
                "LC" => all_mutants.extend(patterns::swap_loop_control_statements(
                    target,
                    root_node,
                    source,
                    nodes::BREAK_STATEMENT,
                    nodes::CONTINUE_STATEMENT,
                    "LC",
                )),
                "BL" => {
                    all_mutants.extend(patterns::flip_boolean_literals_by_kind(
                        target,
                        root_node,
                        source,
                        nodes::BOOLEAN,
                        "BL",
                    ));
                }

                // Shared operator shuffles via common patterns
                "AOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["+", "-", "*", "/"],
                    "AOS",
                )),
                "AAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["+=", "-=", "*=", "/="],
                    "AAOS",
                )),
                "BOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["&", "|", "^"],
                    "BOS",
                )),
                "BAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["&=", "|=", "^="],
                    "BAOS",
                )),
                "COS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["==", "!=", "<", "<=", ">", ">="],
                    "COS",
                )),
                "DOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["/", "~/", "^/"],
                    "DOS",
                )),
                "DAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["/=", "~/=", "^/="],
                    "DAOS",
                )),
                "LOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["&&", "||"],
                    "LOS",
                )),
                "MOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["%", "~%", "^%"],
                    "MOS",
                )),
                "MAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["%=", "~%=", "^%="],
                    "MAOS",
                )),
                "SOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["<<", ">>", "~>>", "^>>"],
                    "SOS",
                )),
                "SAOS" => all_mutants.extend(patterns::shuffle_operators_in_expressions(
                    target,
                    root_node,
                    source,
                    &[nodes::EXPRESSION],
                    &["<<=", ">>=", "~>>=", "^>>="],
                    "SAOS",
                )),
                _ => {
                    panic!(
                        "Unknown mutation slug encountered in FunC engine: {}",
                        m.slug
                    );
                }
            }
        }

        all_mutants
    }

    /// Get all unique mutation slugs
    pub fn get_all_slugs(&self) -> Vec<String> {
        let mut slugs: Vec<String> = self.mutations.iter().map(|m| m.slug.to_string()).collect();
        slugs.sort();
        slugs.dedup();
        slugs
    }

    /// Get the severity for a mutation slug
    pub fn get_severity_by_slug(&self, slug: &str) -> Option<MutationSeverity> {
        self.mutations
            .iter()
            .find(|m| m.slug == slug)
            .map(|m| m.severity.clone())
    }
}

impl MutationEngine for FuncMutationEngine {
    fn get_mutations(&self) -> &[Mutation] {
        <FuncMutationEngine>::get_mutations(self)
    }

    fn apply_all_mutations(&self, target: &Target) -> Vec<Mutant> {
        <FuncMutationEngine>::apply_all_mutations(self, target)
    }

    fn get_all_slugs(&self) -> Vec<String> {
        <FuncMutationEngine>::get_all_slugs(self)
    }

    fn get_severity_by_slug(&self, slug: &str) -> Option<MutationSeverity> {
        <FuncMutationEngine>::get_severity_by_slug(self, slug)
    }
}

// Legacy bespoke helpers removed in favor of common helpers

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Hash;
    use std::collections::{BTreeSet, HashSet};
    use std::path::PathBuf;

    #[test]
    fn no_duplicate_slugs_in_combined_mutations() {
        let engine = FuncMutationEngine::new();
        let mut seen: HashSet<&str> = HashSet::new();
        let mut dups: BTreeSet<String> = BTreeSet::new();
        for m in engine.get_mutations() {
            if !seen.insert(m.slug) {
                dups.insert(m.slug.to_string());
            }
        }
        assert!(
            dups.is_empty(),
            "Duplicate mutation slugs found in FunC engine: {dups:?}",
        );
    }

    #[test]
    fn all_defined_slugs_have_match_arms() {
        // Use a known-good FunC program so parsing succeeds
        let text: &str = include_str!("../../..//tests/examples/func/hello-world.fc");
        let target = Target {
            id: 0,
            path: PathBuf::from("tests/examples/func/hello-world.fc"),
            file_hash: Hash::digest(text.to_string()),
            text: text.to_string(),
            language: Language::FunC,
        };
        let engine = FuncMutationEngine::new();
        // Will panic if any slug is missing a match arm (default case)
        let _ = engine.apply_all_mutations(&target);
    }
}
