use anyhow::Result;
use tokio::io::{stdin, stdout};
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};
use crate::ast::{DelimiterError, Language, RustLanguage};
use crate::repair::balance_file;
use std::sync::Mutex;

struct Backend {
    client: tower_lsp::Client,
    diagnostics: Mutex<Vec<Diagnostic>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["patch-ts.balance".to_string()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "patch-ts LSP server started")
            .await;
    }

    async fn shutdown(&self) -> LspResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        self.publish_diagnostics(uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.publish_diagnostics(uri, &change.text).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        // Use synchronous std::fs to avoid async file system issues
        if let Ok(path) = uri.to_file_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.publish_diagnostics(uri, &content).await;
            }
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let diagnostics = self.diagnostics.lock().unwrap().clone();
        if diagnostics.is_empty() {
            return Ok(None);
        }
        let action = CodeAction {
            title: "Fix unbalanced delimiters with patch-ts".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(diagnostics.clone()),
            command: Some(Command {
                title: "patch-ts balance".to_string(),
                command: "patch-ts.balance".to_string(),
                arguments: Some(vec![serde_json::json!({ "uri": uri.to_string() })]),
            }),
            ..Default::default()
        };
        Ok(Some(vec![CodeActionOrCommand::CodeAction(action)]))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> LspResult<Option<serde_json::Value>> {
        if params.command == "patch-ts.balance" {
            for arg in &params.arguments {
                if let Some(uri_str) = arg.get("uri").and_then(|v| v.as_str()) {
                    if let Ok(uri) = uri_str.parse::<Url>() {
                        if let Ok(path) = uri.to_file_path() {
                            let mut lang = RustLanguage::new();
                            if let Ok(result) = balance_file(&path, None, false, &mut lang, None, 10) {
                                if result.success {
                                    self.client.log_message(MessageType::INFO, "File balanced successfully").await;
                                    if let Ok(content) = std::fs::read_to_string(&path) {
                                        self.publish_diagnostics(uri, &content).await;
                                    }
                                } else {
                                    self.client.show_message(MessageType::ERROR, "Balance failed").await;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}

impl Backend {
    fn new(client: tower_lsp::Client) -> Self {
        Self { client, diagnostics: Mutex::new(vec![]) }
    }

    async fn publish_diagnostics(&self, uri: Url, content: &str) {
        let mut lang = RustLanguage::new();
        let parse_result = lang.parse(content);
        let errors: Vec<DelimiterError> = lang.find_delimiter_errors(&parse_result);

        let diagnostics: Vec<Diagnostic> = errors.iter().map(|e| {
            let (message, span) = match e {
                DelimiterError::Extra { span, delimiter } => {
                    (format!("Extra '{}'", delimiter), span)
                }
                DelimiterError::Missing { expected, insert_at, .. } => {
                    (format!("Missing '{}'", expected), insert_at)
                }
            };
            let start = Position::new(span.start_line as u32 - 1, span.start_column as u32 - 1);
            let end = Position::new(span.end_line as u32 - 1, span.end_column as u32 - 1);
            Diagnostic {
                range: Range { start, end },
                severity: Some(DiagnosticSeverity::ERROR),
                message,
                source: Some("patch-ts".to_string()),
                ..Default::default()
            }
        }).collect();

        *self.diagnostics.lock().unwrap() = diagnostics.clone();
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

pub async fn run_lsp() -> Result<()> {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
