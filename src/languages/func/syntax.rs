pub mod nodes {
    pub const ASSIGNMENT_STATEMENT: &str = "assignment_statement";
    pub const CALL_EXPRESSION: &str = "call_expression";
    pub const DO_STATEMENT: &str = "do_statement";
    pub const EXPRESSION_STATEMENT: &str = "expression_statement";
    pub const FUNCTION_APPLICATION: &str = "function_application";
    pub const IDENTIFIER: &str = "identifier";
    pub const IF_STATEMENT: &str = "if_statement";
    // tree-sitter-func models both `if` and `ifnot` as `if_statement`
    pub const IFNOT_STATEMENT: &str = "if_statement";
    pub const METHOD_CALL: &str = "method_call";
    pub const REPEAT_STATEMENT: &str = "repeat_statement";
    pub const RETURN_STATEMENT: &str = "return_statement";
    pub const UNTIL_STATEMENT: &str = "until_statement";
    pub const VARIABLE_DECLARATION: &str = "variable_declaration";
    pub const WHILE_STATEMENT: &str = "while_statement";
    pub const EXPRESSION: &str = "expression";
    pub const BOOLEAN: &str = "boolean";

    // Statements and controls
    pub const BREAK_STATEMENT: &str = "break_statement";
    pub const CONTINUE_STATEMENT: &str = "continue_statement";
}

pub mod fields {
    // tree-sitter-func exposes both spellings in some productions; support both.
    pub const ARGUMENTS: &str = "arguments";
    pub const AGRUMENTS: &str = "agruments";

    // Common field names used by patterns
    pub const CONDITION: &str = "condition";
    pub const COUNT: &str = "count";
    pub const POSTCONDITION: &str = "postcondition";
}
