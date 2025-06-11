use rue_lexer::Lexer;
use rue_parser::{parse, ParseError};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
pub struct RueLanguageServer {
    client: Client,
    documents: RwLock<HashMap<Url, String>>,
}

impl RueLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    async fn parse_document(&self, _uri: &Url, text: &str) -> Vec<Diagnostic> {
        let mut lexer = Lexer::new(text);
        let tokens = lexer.tokenize();

        match parse(tokens) {
            Ok(_) => Vec::new(), // No errors
            Err(error) => vec![self.parse_error_to_diagnostic(error)],
        }
    }

    fn parse_error_to_diagnostic(&self, error: ParseError) -> Diagnostic {
        // For now, just use character offsets. We could convert to line/column later.
        let range = Range {
            start: Position {
                line: 0,
                character: error.span.start as u32,
            },
            end: Position {
                line: 0,
                character: error.span.end as u32,
            },
        };

        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("rue-lsp".to_string()),
            message: error.message,
            related_information: None,
            tags: None,
            data: None,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for RueLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "rue-language-server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Rue Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        // Store document
        self.documents
            .write()
            .await
            .insert(uri.clone(), text.clone());

        // Parse and send diagnostics
        let diagnostics = self.parse_document(&uri, &text).await;
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;

            // Update stored document
            self.documents
                .write()
                .await
                .insert(uri.clone(), text.clone());

            // Parse and send diagnostics
            let diagnostics = self.parse_document(&uri, &text).await;
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Remove document from storage
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);

        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(RueLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use rue_lexer::Lexer;
    use rue_parser::parse;

    #[test]
    fn test_while_loop_parsing() {
        let text = r#"
fn test_while(n) {
    while n > 0 {
        n + 1
    }
    42
}

fn main() {
    test_while(5)
}
"#;

        let mut lexer = Lexer::new(text);
        let tokens = lexer.tokenize();
        let result = parse(tokens);

        assert!(result.is_ok(), "While loop should parse without errors");
    }

    #[test]
    fn test_invalid_while_syntax() {
        let text = r#"
fn test_invalid() {
    while {
        42
    }
}
"#;

        let mut lexer = Lexer::new(text);
        let tokens = lexer.tokenize();
        let result = parse(tokens);

        assert!(
            result.is_err(),
            "Invalid while syntax should produce errors"
        );
    }
}
