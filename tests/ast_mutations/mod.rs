use muton::mutations::func::ast_engine::ASTMutationEngine;
use muton::types::{Language as MutationLanguage, Target};
use std::path::Path;
use tempfile::tempdir;

mod test_error_replacement;
mod test_comment_replacement; 
mod test_conditional_mutations;
mod test_argument_swap;
mod test_variable_mutations;
mod test_loop_mutations;

mod test_edge_cases;

/// Helper function to create a test FunC file
pub fn create_test_file(content: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.fc");
    std::fs::write(&file_path, content).expect("Failed to write test file");
    (temp_dir, file_path)
}

/// Helper function to create a Target from test content
pub fn create_test_target(content: &str) -> (tempfile::TempDir, Target) {
    let (temp_dir, file_path) = create_test_file(content);
    let target = Target::new(file_path.to_string_lossy().to_string(), MutationLanguage::FunC)
        .expect("Failed to create target");
    (temp_dir, target)
}

/// Helper function to get AST mutation engine
pub fn get_ast_engine() -> ASTMutationEngine {
    ASTMutationEngine::new(&MutationLanguage::FunC)
} 