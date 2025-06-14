use rue_ast::CstRoot;
use rue_codegen::compile_to_executable;
use rue_parser::ParseError;
use rue_semantic::{SemanticError, analyze_cst};
use std::sync::Arc;

// Input structs
#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub path: String,
    #[return_ref]
    pub text: String,
}

// Tracked functions
#[salsa::tracked]
pub fn parse_file(
    db: &dyn salsa::Database,
    file: SourceFile,
) -> Result<Arc<CstRoot>, Arc<ParseError>> {
    let text = file.text(db);
    let mut lexer = rue_lexer::Lexer::new(text.as_str());
    let tokens = lexer.tokenize();

    match rue_parser::parse(tokens) {
        Ok(cst) => Ok(Arc::new(cst)),
        Err(e) => Err(Arc::new(e)),
    }
}

#[salsa::tracked]
pub fn analyze_file(
    db: &dyn salsa::Database,
    file: SourceFile,
) -> Result<Arc<rue_semantic::Scope>, Arc<SemanticError>> {
    // Parse the file first
    let ast = match parse_file(db, file) {
        Ok(ast) => ast,
        Err(parse_error) => {
            return Err(Arc::new(SemanticError {
                message: format!("Parse error: {}", parse_error.message),
                span: parse_error.span,
            }));
        }
    };

    // Analyze the AST
    match analyze_cst(&ast) {
        Ok(scope) => Ok(Arc::new(scope)),
        Err(e) => Err(Arc::new(e)),
    }
}

// Re-export Salsa's default database implementation
pub type RueDatabase = salsa::DatabaseImpl;

#[cfg(test)]
mod tests {
    use super::*;
    use salsa::Setter;

    #[test]
    fn test_parse_file() {
        let mut db = RueDatabase::default();

        // Create a source file
        let file = SourceFile::new(&db, "test.rue".to_string(), "fn main() { 42 }".to_string());

        // Parse it
        let result = parse_file(&db, file);
        assert!(result.is_ok());

        // Modify the file
        file.set_text(&mut db).to("fn main() { 2 + 3 }".to_string());

        // Parse again (Salsa will recompute)
        let result = parse_file(&db, file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incremental_parsing() {
        let db = RueDatabase::default();

        // Create a source file
        let file = SourceFile::new(
            &db,
            "factorial.rue".to_string(),
            r#"
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}"#
            .to_string(),
        );

        // Parse it
        let result = parse_file(&db, file);
        assert!(result.is_ok());

        // Parse again without changes (should be cached)
        let result2 = parse_file(&db, file);
        assert!(result.is_ok());
        assert!(Arc::ptr_eq(&result.unwrap(), &result2.unwrap())); // Same Arc = cached
    }

    #[test]
    fn test_semantic_analysis_simple() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn main() {
    42
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_ok());

        let scope = result.unwrap();
        assert!(scope.functions.contains_key("main"));
        assert_eq!(scope.functions["main"].param_count, 0);
        assert_eq!(
            scope.functions["main"].return_type,
            rue_semantic::RueType::I64
        );
    }

    #[test]
    fn test_semantic_analysis_with_parameter() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_ok());

        let scope = result.unwrap();
        assert!(scope.functions.contains_key("factorial"));
        assert_eq!(scope.functions["factorial"].param_count, 1);
    }

    #[test]
    fn test_semantic_analysis_undefined_variable() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn main() {
    undefined_var
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined variable: undefined_var"));
    }

    #[test]
    fn test_semantic_analysis_undefined_function() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn main() {
    undefined_func(42)
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined function: undefined_func"));
    }

    #[test]
    fn test_semantic_analysis_wrong_argument_count() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn factorial(n) {
    n
}

fn main() {
    factorial()
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.message.contains("expects 1 arguments, got 0"));
    }

    #[test]
    fn test_semantic_analysis_let_statement() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn main() {
    let x = 42;
    x + 1
}
"#
            .to_string(),
        );

        let result = analyze_file(&db, file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_simple_program() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "test.rue".to_string(),
            r#"
fn main() {
    42
}
"#
            .to_string(),
        );

        let result = compile_file(&db, file);
        assert!(result.is_ok());

        let executable = result.unwrap();
        // Should be a valid ELF executable
        assert_eq!(&executable[0..4], &[0x7f, 0x45, 0x4c, 0x46]);
    }

    #[test]
    fn test_compile_factorial() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "factorial.rue".to_string(),
            r#"
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    factorial(5)
}
"#
            .to_string(),
        );

        let result = compile_file(&db, file);
        assert!(result.is_ok());

        let executable = result.unwrap();
        // Should be a valid ELF executable
        assert_eq!(&executable[0..4], &[0x7f, 0x45, 0x4c, 0x46]);
        assert!(executable.len() > 200); // Should be reasonable size
    }

    #[test]
    fn test_compile_assignment() {
        let db = RueDatabase::default();

        let file = SourceFile::new(
            &db,
            "assignment.rue".to_string(),
            r#"
fn main() {
    let x = 42;
    x = 100;
    x
}
"#
            .to_string(),
        );

        let result = compile_file(&db, file);
        if let Err(ref e) = result {
            println!("Compilation error: {}", e.message);
        }
        assert!(result.is_ok());

        let executable = result.unwrap();
        // Should be a valid ELF executable
        assert_eq!(&executable[0..4], &[0x7f, 0x45, 0x4c, 0x46]);
        println!("Executable length: {}", executable.len());
        assert!(executable.len() > 100); // Should be reasonable size
    }
}

// Simplified compilation error for Salsa
#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub message: String,
}

#[salsa::tracked]
pub fn compile_file(
    db: &dyn salsa::Database,
    file: SourceFile,
) -> Result<Arc<Vec<u8>>, Arc<CompileError>> {
    // Parse and analyze the file first
    let scope = match analyze_file(db, file) {
        Ok(scope) => scope,
        Err(semantic_error) => {
            return Err(Arc::new(CompileError {
                message: format!("Semantic error: {}", semantic_error.message),
            }));
        }
    };

    let ast = match parse_file(db, file) {
        Ok(ast) => ast,
        Err(parse_error) => {
            return Err(Arc::new(CompileError {
                message: format!("Parse error: {}", parse_error.message),
            }));
        }
    };

    // Generate executable
    match compile_to_executable(&ast, &scope) {
        Ok(executable) => Ok(Arc::new(executable)),
        Err(e) => Err(Arc::new(CompileError { message: e.message })),
    }
}
