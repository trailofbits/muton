/// Node type names from tree-sitter-tolk grammar
pub mod nodes {
    // Statements
    pub const IF_STATEMENT: &str = "if_statement";
    pub const RETURN_STATEMENT: &str = "return_statement";
    pub const EXPRESSION_STATEMENT: &str = "expression_statement";
    pub const WHILE_STATEMENT: &str = "while_statement";
    pub const DO_WHILE_STATEMENT: &str = "do_while_statement";
    pub const REPEAT_STATEMENT: &str = "repeat_statement";
    pub const THROW_STATEMENT: &str = "throw_statement";
    pub const ASSERT_STATEMENT: &str = "assert_statement";
    pub const TRY_CATCH_STATEMENT: &str = "try_catch_statement";
    pub const MATCH_STATEMENT: &str = "match_statement";
    pub const BLOCK_STATEMENT: &str = "block_statement";
    pub const BREAK_STATEMENT: &str = "break_statement";
    pub const CONTINUE_STATEMENT: &str = "continue_statement";

    // Expressions
    pub const FUNCTION_CALL: &str = "function_call";
    pub const BINARY_OPERATOR: &str = "binary_operator";
    pub const UNARY_OPERATOR: &str = "unary_operator";
    pub const BOOLEAN_LITERAL: &str = "boolean_literal";
    pub const ASSIGNMENT: &str = "assignment";
    pub const LAMBDA_EXPRESSION: &str = "lambda_expression";
    pub const MATCH_EXPRESSION: &str = "match_expression";
}

/// Field names from tree-sitter-tolk grammar
pub mod fields {
    // Control flow fields
    pub const CONDITION: &str = "condition";
    pub const BODY: &str = "body";
    pub const ALTERNATIVE: &str = "alternative";

    // Function call fields
    pub const FUNCTION_NAME: &str = "function_name";
    pub const ARGUMENTS: &str = "arguments";

    // Binary operator fields
    pub const OPERATOR_NAME: &str = "operator_name";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";

    // Other fields
    pub const EXPR: &str = "expr";
    pub const VALUE: &str = "value";
}
