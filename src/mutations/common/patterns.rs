use crate::mutations::common::utils::{
    calculate_line_offset, create_mutant, is_in_comment, node_text, visit_nodes_with_cursor,
};
use crate::types::{Mutant, Target};
use tree_sitter::Node;

/// Replace a specific child field on nodes of a given kind with fixed text
pub fn replace_field_for_nodes_of_kind(
    target: &Target,
    root: Node,
    source: &str,
    node_kind: &str,
    field_name: &str,
    slug: &str,
    new_text: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if node.kind() == node_kind
            && !is_in_comment(&node)
            && let Some(field_node) = node.child_by_field_name(field_name)
        {
            mutants.push(create_mutant(
                target,
                &field_node,
                source,
                slug,
                new_text.to_string(),
            ));
        }
    });
    mutants
}

/// Replace entire nodes of the provided kinds with a fixed replacement text
/// controlled by a filter predicate
pub fn replace_entire_nodes_of_kinds_filtered(
    target: &Target,
    root: Node,
    source: &str,
    node_kinds: &[&str],
    slug: &str,
    replacement_text: &str,
    should_replace: &dyn Fn(&Node, &str) -> bool,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let kinds: Vec<&str> = node_kinds.to_vec();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if kinds.contains(&node.kind())
            && !is_in_comment(&node)
            && !has_ancestor_with_kind(&node, &kinds)
            && should_replace(&node, source)
        {
            mutants.push(create_mutant(
                target,
                &node,
                source,
                slug,
                replacement_text.to_string(),
            ));
        }
    });
    mutants
}

/// Swap adjacent arguments inside a child field (e.g., "arguments") for specified node kinds
pub fn swap_adjacent_arguments_for_kinds(
    target: &Target,
    root: Node,
    source: &str,
    node_kinds: &[&str],
    args_field_name: &str,
    slug: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let kinds: Vec<&str> = node_kinds.to_vec();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if kinds.contains(&node.kind())
            && !is_in_comment(&node)
            && let Some(args_node) = node.child_by_field_name(args_field_name)
        {
            let mut args: Vec<Node> = Vec::new();
            let mut ac = args_node.walk();
            for child in args_node.children(&mut ac) {
                let k = child.kind();
                if k != "(" && k != ")" && k != "," {
                    args.push(child);
                }
            }
            if args.len() >= 2 {
                for i in 0..args.len() - 1 {
                    let a = args[i];
                    let b = args[i + 1];
                    let start = a.start_byte();
                    let end = b.end_byte();
                    let a_text = node_text(&a, source);
                    let b_text = node_text(&b, source);
                    let full_text = &source[start..end];
                    let swapped = format!("{b_text}, {a_text}");
                    mutants.push(Mutant {
                        id: 0,
                        target_id: target.id,
                        mutation_slug: slug.to_string(),
                        byte_offset: start as u32,
                        line_offset: calculate_line_offset(source, start),
                        old_text: full_text.to_string(),
                        new_text: swapped,
                    });
                }
            }
        }
    });
    mutants
}

/// Flip boolean literal nodes of a specific kind by replacing "true" <-> "false"
pub fn flip_boolean_literals_by_kind(
    target: &Target,
    root: Node,
    source: &str,
    boolean_node_kind: &str,
    slug: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if node.kind() == boolean_node_kind && !is_in_comment(&node) {
            let old = node_text(&node, source);
            let new = if old == "true" { "false" } else { "true" };
            mutants.push(create_mutant(target, &node, source, slug, new.to_string()));
        }
    });
    mutants
}

/// Shuffle operator tokens inside expressions of specified kinds by replacing any occurrence
/// of the provided operators with any other in the set (excluding identity)
pub fn shuffle_operators_in_expressions(
    target: &Target,
    root: Node,
    source: &str,
    expr_node_kinds: &[&str],
    operators: &[&str],
    slug: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let kinds: Vec<&str> = expr_node_kinds.to_vec();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if kinds.contains(&node.kind()) && !is_in_comment(&node) {
            let mut nc = node.walk();
            for child in node.children(&mut nc) {
                let token = node_text(&child, source);
                if operators.contains(&token) {
                    for replacement in operators.iter().copied() {
                        if replacement != token {
                            mutants.push(Mutant {
                                id: 0,
                                target_id: target.id,
                                mutation_slug: slug.to_string(),
                                byte_offset: child.start_byte() as u32,
                                line_offset: calculate_line_offset(source, child.start_byte()),
                                old_text: token.to_string(),
                                new_text: replacement.to_string(),
                            });
                        }
                    }
                }
            }
        }
    });
    mutants
}

/// Wrap entire nodes of the provided kinds with arbitrary prefix/suffix around the old text
pub fn wrap_nodes_of_kinds_with_wrappers(
    target: &Target,
    root: Node,
    source: &str,
    node_kinds: &[&str],
    slug: &str,
    prefix: &str,
    suffix: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let kinds: Vec<&str> = node_kinds.to_vec();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if kinds.contains(&node.kind())
            && !is_in_comment(&node)
            && !has_ancestor_with_kind(&node, &kinds)
        {
            let old = node_text(&node, source);
            let replacement = format!("{prefix}{old}{suffix}");
            mutants.push(create_mutant(target, &node, source, slug, replacement));
        }
    });
    mutants
}

// New helpers and utilities

fn is_punctuation_kind(kind: &str) -> bool {
    kind == "(" || kind == ")" || kind == ","
}

fn is_keyword_kind(kind: &str, keywords: &[&str]) -> bool {
    keywords.contains(&kind)
}

fn has_ancestor_with_kind(node: &Node, kinds: &[&str]) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if kinds.contains(&parent.kind()) {
            return true;
        }
        current = parent.parent();
    }
    false
}

fn first_named_child_after_keyword<'a>(node: &Node<'a>, keywords: &[&str]) -> Option<Node<'a>> {
    let mut c = node.walk();
    for child in node.children(&mut c) {
        if child.is_missing() || child.is_error() {
            continue;
        }
        let k = child.kind();
        if is_keyword_kind(k, keywords) || is_punctuation_kind(k) {
            continue;
        }
        if child.is_named() {
            return Some(child);
        }
    }
    None
}

/// Replace a condition for nodes of a specific kind using field-first, positional-fallback
#[allow(clippy::too_many_arguments)]
pub fn replace_condition_for_nodes_of_kind(
    target: &Target,
    root: Node,
    source: &str,
    node_kind: &str,
    condition_field_name: &str,
    keyword_kinds: &[&str],
    slug: &str,
    replacement: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if node.kind() == node_kind && !is_in_comment(&node) {
            if let Some(field_node) = node.child_by_field_name(condition_field_name) {
                let old_text = node_text(&field_node, source);
                let trimmed_start = old_text.trim_start();
                let trimmed_end = old_text.trim_end();
                let needs_parens = trimmed_start.starts_with('(') && trimmed_end.ends_with(')');
                let new_text = if needs_parens {
                    format!("({replacement})")
                } else {
                    replacement.to_string()
                };
                mutants.push(create_mutant(target, &field_node, source, slug, new_text));
            } else if let Some(cond) = first_named_child_after_keyword(&node, keyword_kinds)
                && cond.kind() != ";"
                && cond.kind() != "{"
            {
                let old_text = node_text(&cond, source);
                let trimmed_start = old_text.trim_start();
                let trimmed_end = old_text.trim_end();
                let needs_parens = trimmed_start.starts_with('(') && trimmed_end.ends_with(')');
                let new_text = if needs_parens {
                    format!("({replacement})")
                } else {
                    replacement.to_string()
                };
                mutants.push(create_mutant(target, &cond, source, slug, new_text));
            }
        }
    });
    mutants
}

/// Replace the repeat count for nodes of a specific kind using field-first, positional-fallback
#[allow(clippy::too_many_arguments)]
pub fn replace_repeat_count_for_nodes_of_kind(
    target: &Target,
    root: Node,
    source: &str,
    node_kind: &str,
    count_field_name: &str,
    keyword_kinds: &[&str],
    slug: &str,
    replacement: &str,
) -> Vec<Mutant> {
    replace_condition_for_nodes_of_kind(
        target,
        root,
        source,
        node_kind,
        count_field_name,
        keyword_kinds,
        slug,
        replacement,
    )
}

/// Replace the first argument for calls whose callee matches a predicate
#[allow(clippy::too_many_arguments)]
pub fn replace_first_argument_for_calls_matching(
    target: &Target,
    root: Node,
    source: &str,
    call_node_kinds: &[&str],
    args_field_name: &str,
    alt_args_kinds: &[&str],
    slug: &str,
    callee_matches: &dyn Fn(&str) -> bool,
    replacement: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let call_kinds: Vec<&str> = call_node_kinds.to_vec();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if call_kinds.contains(&node.kind()) && !is_in_comment(&node) {
            let callee_text = if let Some(callee_node) = node.child(0) {
                node_text(&callee_node, source)
            } else {
                return;
            };
            if !callee_matches(callee_text) {
                return;
            }
            let args_node_opt = node.child_by_field_name(args_field_name).or_else(|| {
                let mut c = node.walk();
                for child in node.children(&mut c) {
                    let k = child.kind();
                    if alt_args_kinds.contains(&k) || k == args_field_name {
                        return Some(child);
                    }
                }
                None
            });
            if let Some(args_node) = args_node_opt {
                let mut ac = args_node.walk();
                for child in args_node.children(&mut ac) {
                    let k = child.kind();
                    if is_punctuation_kind(k) {
                        continue;
                    }
                    mutants.push(create_mutant(
                        target,
                        &child,
                        source,
                        slug,
                        replacement.to_string(),
                    ));
                    break;
                }
            }
        }
    });
    mutants
}

/// Swap loop control statements for any language by node kind strings
pub fn swap_loop_control_statements(
    target: &Target,
    root: Node,
    source: &str,
    break_kind: &str,
    continue_kind: &str,
    slug: &str,
) -> Vec<Mutant> {
    let mut mutants = Vec::new();
    let mut cursor = root.walk();
    visit_nodes_with_cursor(root, &mut cursor, &mut |node| {
        if is_in_comment(&node) {
            return;
        }
        let kind = node.kind();
        if kind == break_kind || kind == continue_kind {
            let old = node_text(&node, source);
            let new = if kind == break_kind {
                old.replace("break", "continue")
            } else {
                old.replace("continue", "break")
            };
            mutants.push(create_mutant(target, &node, source, slug, new));
        }
    });
    mutants
}
