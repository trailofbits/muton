use crate::types::{Mutant, Target};
use tree_sitter::{Node, TreeCursor};

/// Get text content of a node from source
pub fn node_text<'a>(node: &Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

/// Check if a node is inside a comment
pub fn is_in_comment(node: &Node) -> bool {
    let kind = node.kind();
    if kind == "comment" {
        return true;
    }

    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "comment" {
            return true;
        }
        current = parent.parent();
    }

    false
}

/// Visit all nodes in the tree with a callback, using a provided cursor for advanced usage
pub fn visit_nodes_with_cursor<F>(node: Node, cursor: &mut TreeCursor, callback: &mut F)
where
    F: FnMut(Node),
{
    callback(node);

    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            visit_nodes_with_cursor(child, cursor, callback);

            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

/// Calculate line offset for a byte position
pub fn calculate_line_offset(source: &str, byte_offset: usize) -> u32 {
    source
        .bytes()
        .take(byte_offset)
        .filter(|&b| b == b'\n')
        .count() as u32
}

/// Common helper to create a mutant with consistent fields
pub fn create_mutant(
    target: &Target,
    node: &Node,
    source: &str,
    slug: &str,
    new_text: String,
) -> Mutant {
    Mutant {
        id: 0,
        target_id: target.id,
        mutation_slug: slug.to_string(),
        byte_offset: node.start_byte() as u32,
        line_offset: calculate_line_offset(source, node.start_byte()),
        old_text: node_text(node, source).to_string(),
        new_text,
    }
}
