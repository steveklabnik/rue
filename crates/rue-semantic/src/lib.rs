use rue_ast::{CstRoot, ExpressionNode, FunctionNode, StatementNode};
use std::collections::HashMap;

// Semantic analysis types
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub span: rue_lexer::Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RueType {
    I64,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scope {
    pub variables: HashMap<String, RueType>,
    pub functions: HashMap<String, FunctionSignature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub param_count: usize,
    pub return_type: RueType,
}

// Semantic analysis functions
pub fn analyze_cst(ast: &CstRoot) -> Result<Scope, SemanticError> {
    let mut scope = Scope::default();

    for item in &ast.items {
        match item {
            rue_ast::CstNode::Function(func) => {
                analyze_function(&mut scope, func)?;
            }
            rue_ast::CstNode::Statement(stmt) => {
                analyze_statement(&mut scope, stmt)?;
            }
            _ => {} // Skip other node types for now
        }
    }

    Ok(scope)
}

// Helper functions for semantic analysis
fn analyze_function(scope: &mut Scope, func: &FunctionNode) -> Result<(), SemanticError> {
    // Extract function name
    let func_name = match &func.name.kind {
        rue_lexer::TokenKind::Ident(name) => name.clone(),
        _ => {
            return Err(SemanticError {
                message: "Expected function name".to_string(),
                span: func.name.span,
            });
        }
    };

    // Check parameter count (rue only supports single parameter for now)
    let param_count = func.param_list.params.len();
    if param_count > 1 {
        return Err(SemanticError {
            message: "Functions can only have at most one parameter".to_string(),
            span: func.param_list.open_paren.span,
        });
    }

    // Register function in scope
    scope.functions.insert(
        func_name,
        FunctionSignature {
            param_count,
            return_type: RueType::I64, // All functions return i64
        },
    );

    // Create local scope for function body
    let mut local_scope = scope.clone();

    // Add parameter to local scope if it exists
    if let Some(param) = func.param_list.params.first() {
        if let rue_lexer::TokenKind::Ident(param_name) = &param.kind {
            local_scope
                .variables
                .insert(param_name.clone(), RueType::I64);
        }
    }

    // Analyze function body
    for stmt in &func.body.statements {
        analyze_statement(&mut local_scope, stmt)?;
    }

    Ok(())
}

fn analyze_statement(scope: &mut Scope, stmt: &StatementNode) -> Result<(), SemanticError> {
    match stmt {
        StatementNode::Let(let_stmt) => {
            // Analyze the value expression
            analyze_expression(scope, &let_stmt.value)?;

            // Add variable to scope
            if let rue_lexer::TokenKind::Ident(var_name) = &let_stmt.name.kind {
                scope.variables.insert(var_name.clone(), RueType::I64);
            }
        }
        StatementNode::Expression(expr) => {
            analyze_expression(scope, expr)?;
        }
        StatementNode::If(if_stmt) => {
            // Analyze condition
            analyze_expression(scope, &if_stmt.condition)?;

            // Analyze then block
            for stmt in &if_stmt.then_block.statements {
                analyze_statement(scope, stmt)?;
            }

            // Analyze else block if it exists
            if let Some(else_clause) = &if_stmt.else_clause {
                match &else_clause.body {
                    rue_ast::ElseBodyNode::Block(block) => {
                        for stmt in &block.statements {
                            analyze_statement(scope, stmt)?;
                        }
                    }
                    rue_ast::ElseBodyNode::If(nested_if) => {
                        analyze_statement(scope, &StatementNode::If(nested_if.clone()))?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn analyze_expression(scope: &Scope, expr: &ExpressionNode) -> Result<RueType, SemanticError> {
    match expr {
        ExpressionNode::Literal(_) => Ok(RueType::I64), // All literals are i64
        ExpressionNode::Identifier(token) => {
            if let rue_lexer::TokenKind::Ident(name) = &token.kind {
                if scope.variables.contains_key(name) {
                    Ok(RueType::I64)
                } else {
                    Err(SemanticError {
                        message: format!("Undefined variable: {}", name),
                        span: token.span,
                    })
                }
            } else {
                Err(SemanticError {
                    message: "Expected identifier".to_string(),
                    span: token.span,
                })
            }
        }
        ExpressionNode::Binary(binary_expr) => {
            // Analyze both operands
            let left_type = analyze_expression(scope, &binary_expr.left)?;
            let right_type = analyze_expression(scope, &binary_expr.right)?;

            // Both operands must be i64
            if left_type == RueType::I64 && right_type == RueType::I64 {
                Ok(RueType::I64)
            } else {
                Err(SemanticError {
                    message: "Binary operators require i64 operands".to_string(),
                    span: binary_expr.operator.span,
                })
            }
        }
        ExpressionNode::Call(call_expr) => {
            // Get function name
            if let ExpressionNode::Identifier(func_token) = &*call_expr.function {
                if let rue_lexer::TokenKind::Ident(func_name) = &func_token.kind {
                    // Check if function exists
                    if let Some(signature) = scope.functions.get(func_name) {
                        // Check argument count
                        if call_expr.args.len() != signature.param_count {
                            return Err(SemanticError {
                                message: format!(
                                    "Function '{}' expects {} arguments, got {}",
                                    func_name,
                                    signature.param_count,
                                    call_expr.args.len()
                                ),
                                span: call_expr.open_paren.span,
                            });
                        }

                        // Analyze all arguments
                        for arg in &call_expr.args {
                            analyze_expression(scope, arg)?;
                        }

                        Ok(signature.return_type.clone())
                    } else {
                        Err(SemanticError {
                            message: format!("Undefined function: {}", func_name),
                            span: func_token.span,
                        })
                    }
                } else {
                    Err(SemanticError {
                        message: "Expected function name".to_string(),
                        span: func_token.span,
                    })
                }
            } else {
                Err(SemanticError {
                    message: "Function calls must use identifiers".to_string(),
                    span: call_expr.open_paren.span,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rue_lexer::Lexer;

    fn parse_and_analyze(source: &str) -> Result<Scope, SemanticError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let ast = rue_parser::parse(tokens).map_err(|e| SemanticError {
            message: format!("Parse error: {}", e.message),
            span: e.span,
        })?;
        analyze_cst(&ast)
    }

    #[test]
    fn test_semantic_analysis_simple() {
        let result = parse_and_analyze(
            r#"
fn main() {
    42
}
"#,
        );
        assert!(result.is_ok());

        let scope = result.unwrap();
        assert!(scope.functions.contains_key("main"));
        assert_eq!(scope.functions["main"].param_count, 0);
        assert_eq!(scope.functions["main"].return_type, RueType::I64);
    }

    #[test]
    fn test_semantic_analysis_with_parameter() {
        let result = parse_and_analyze(
            r#"
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
"#,
        );
        assert!(result.is_ok());

        let scope = result.unwrap();
        assert!(scope.functions.contains_key("factorial"));
        assert_eq!(scope.functions["factorial"].param_count, 1);
    }

    #[test]
    fn test_semantic_analysis_undefined_variable() {
        let result = parse_and_analyze(
            r#"
fn main() {
    undefined_var
}
"#,
        );
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined variable: undefined_var"));
    }

    #[test]
    fn test_semantic_analysis_undefined_function() {
        let result = parse_and_analyze(
            r#"
fn main() {
    undefined_func(42)
}
"#,
        );
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined function: undefined_func"));
    }

    #[test]
    fn test_semantic_analysis_wrong_argument_count() {
        let result = parse_and_analyze(
            r#"
fn factorial(n) {
    n
}

fn main() {
    factorial()
}
"#,
        );
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("expects 1 arguments, got 0"));
    }

    #[test]
    fn test_semantic_analysis_let_statement() {
        let result = parse_and_analyze(
            r#"
fn main() {
    let x = 42
    x + 1
}
"#,
        );
        assert!(result.is_ok());
    }
}
