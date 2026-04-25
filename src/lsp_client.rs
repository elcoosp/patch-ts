use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// A lightweight, synchronous LSP client for collecting diagnostics.
pub struct LspClient {
    child: Child,
    reader: BufReader<std::process::ChildStdout>,
    writer: std::process::ChildStdin,
    next_id: u64,
}

#[derive(Debug, Serialize)]
struct LspRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

#[derive(Debug, Deserialize)]
struct LspResponse {
    #[serde(default)]
    id: Option<u64>,
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    error: Option<Value>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    params: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct PublishDiagnosticsParams {
    uri: String,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<u32>,
    pub message: String,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl LspClient {
    pub fn new(cmd: &str, args: &[&str], root_uri: &str) -> Result<Self> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|| format!("Failed to spawn language server: {}", cmd))?;

        let writer = child.stdin.take().unwrap();
        let reader = BufReader::new(child.stdout.take().unwrap());

        let mut client = Self {
            child,
            reader,
            writer,
            next_id: 1,
        };

        client.initialize(root_uri)?;
        Ok(client)
    }

    fn initialize(&mut self, root_uri: &str) -> Result<()> {
        let params = serde_json::json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "diagnostic": {
                        "dynamicRegistration": false
                    }
                }
            }
        });
        let _resp = self.request("initialize", params)?;
        self.send_notification("initialized", serde_json::json!({}));
        Ok(())
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let req = LspRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        };
        let req_json = serde_json::to_string(&req)?;
        writeln!(self.writer, "Content-Length: {}\r\n\r\n{}", req_json.len(), req_json)?;
        self.writer.flush()?;

        let mut content_length = 0usize;
        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line)?;
            if line == "\r\n" { break; }
            if line.to_lowercase().starts_with("content-length:") {
                content_length = line.trim_start_matches("Content-Length:").trim().parse()?;
            }
        }

        let mut body = vec![0u8; content_length];
        self.reader.read_exact(&mut body)?;
        let resp: LspResponse = serde_json::from_slice(&body)?;
        if let Some(e) = resp.error {
            anyhow::bail!("LSP error: {:?}", e);
        }
        Ok(resp.result.unwrap_or(Value::Null))
    }

    fn send_notification(&mut self, method: &str, params: Value) {
        let notif = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        let json = notif.to_string();
        let _ = writeln!(self.writer, "Content-Length: {}\r\n\r\n{}", json.len(), json);
        let _ = self.writer.flush();
    }

    pub fn get_diagnostics(
        &mut self,
        uri: &str,
        content: &str,
        language_id: &str,
        timeout: Duration,
    ) -> Result<Vec<Diagnostic>> {
        self.send_notification(
            "textDocument/didOpen",
            serde_json::json!({
                "textDocument": {
                    "uri": uri,
                    "languageId": language_id,
                    "version": 1,
                    "text": content,
                }
            }),
        );

        // Try pulling diagnostics (LSP 3.17)
        if let Ok(resp) = self.request(
            "textDocument/diagnostic",
            serde_json::json!({
                "textDocument": { "uri": uri }
            }),
        ) {
            if let Some(items) = resp.get("items").and_then(|v| v.as_array()) {
                return Ok(serde_json::from_value(Value::Array(items.clone()))?);
            }
        }

        // Fallback: listen for publishDiagnostics notifications
        let start = std::time::Instant::now();
        let mut diagnostics = Vec::new();
        while start.elapsed() < timeout {
            if self.child.try_wait()?.is_some() {
                break;
            }
            let mut line = String::new();
            if let Ok(0) = self.reader.read_line(&mut line) {
                // EOF reached
                break;
            }
            if line.starts_with("Content-Length:") {
                let content_length: usize = line.trim_start_matches("Content-Length:").trim().parse()?;
                // Read the empty line after header
                let mut empty = String::new();
                self.reader.read_line(&mut empty)?;
                let mut body = vec![0u8; content_length];
                self.reader.read_exact(&mut body)?;
                if let Ok(msg) = serde_json::from_slice::<LspResponse>(&body) {
                    if msg.method.as_deref() == Some("textDocument/publishDiagnostics") {
                        if let Some(params) = msg.params {
                            if let Ok(p) = serde_json::from_value::<PublishDiagnosticsParams>(params) {
                                diagnostics.extend(p.diagnostics);
                            }
                        }
                    }
                }
            }
        }
        Ok(diagnostics)
    }

    pub fn shutdown(mut self) -> Result<()> {
        let _ = self.child.kill();
        let _ = self.child.wait();
        Ok(())
    }
}
