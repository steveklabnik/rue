use rue_ast::CstRoot;
use rue_parser::ParseError;
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
        let file = SourceFile::new(&db, "test.rue".to_string(), "42".to_string());

        // Parse it
        let result = parse_file(&db, file);
        assert!(result.is_ok());

        // Modify the file
        file.set_text(&mut db).to("2 + 3".to_string());

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
}
