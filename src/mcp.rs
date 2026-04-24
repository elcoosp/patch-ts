use crate::ast::RustLanguage;
use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PatchParams {
    pub file: String,
    pub line: usize,
    pub old: String,
    pub new: String,
    #[serde(default = "default_fuzz")]
    pub fuzz: usize,
    #[serde(default = "default_confidence")]
    pub confidence: f64,
    pub dry_run: Option<bool>,
    pub force: Option<bool>,
}
fn default_fuzz() -> usize { 5 }
fn default_confidence() -> f64 { 0.9 }

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct BalanceParams {
    pub file: String,
    pub apply: Option<bool>,
    pub max_cost: Option<usize>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ExplainParams {
    pub file: String,
    pub line: usize,
}

fn to_schema<T: schemars::JsonSchema>() -> Value {
    let schema = schemars::schema_for!(T);
    serde_json::to_value(schema).unwrap_or(Value::Null)
}

pub fn run_mcp() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }
        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let err = json!({"jsonrpc":"2.0","error":{"code":-32700,"message":format!("Parse error: {}",e)},"id":null});
                writeln!(stdout, "{}", serde_json::to_string(&err)?)?;
                stdout.flush()?;
                continue;
            }
        };
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let response = match method {
            "tools/list" => {
                let tools = vec![
                    json!({"name":"patch","description":"Apply a patch","inputSchema": to_schema::<PatchParams>()}),
                    json!({"name":"balance","description":"Fix unbalanced delimiters","inputSchema": to_schema::<BalanceParams>()}),
                    json!({"name":"explain","description":"Explain syntax error","inputSchema": to_schema::<ExplainParams>()}),
                ];
                json!({"jsonrpc":"2.0","id":id,"result":{"tools":tools}})
            },
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(json!({}));
                match tool_name {
                    "patch" => handle_patch_tool(arguments),
                    "balance" => handle_balance_tool(arguments),
                    "explain" => handle_explain_tool(arguments),
                    _ => json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":format!("Unknown tool: {}", tool_name)}}),
                }
            },
            "resources/list" => {
                let resources = vec![
                    json!({"uri":"patch-ts://symbols/main.rs","name":"Symbols"}),
                    json!({"uri":"patch-ts://history","name":"Patch history"}),
                    json!({"uri":"patch-ts://provenance","name":"Provenance records"}),
                    json!({"uri":"patch-ts://dashboard","name":"Dashboard","mimeType":"text/html"}),
                    json!({"uri":"patch-ts://review","name":"Code Review","mimeType":"text/html"}),
                ];
                json!({"jsonrpc":"2.0","id":id,"result":{"resources":resources}})
            },
            "resources/read" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let uri = params.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                let content = match uri {
                    "patch-ts://symbols/main.rs" => {
                        if let Ok(source) = std::fs::read_to_string("main.rs") {
                            crate::symbols::build_index(&source, "rs").map(|idx| json!(idx)).unwrap_or(json!({}))
                        } else { json!({"error":"File not found"}) }
                    },
                    "patch-ts://history" => {
                        let mgr = crate::history::HistoryManager::new();
                        json!(mgr.list().unwrap_or_default())
                    },
                    "patch-ts://provenance" => {
                        json!(crate::provenance::query_provenance(None,None).unwrap_or_default())
                    },
                    "patch-ts://dashboard" | "patch-ts://review" => {
                        let state = crate::dashboard::DashboardState {
                            patch_id: "latest".to_string(),
                            diff: None, score: None, provenance: None, gate_result: None, suggestions: None,
                        };
                        let html = crate::dashboard::render_dashboard(&state);
                        json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":html,"mimeType":"text/html"}]}})
                    },
                    _ => json!({"error":"Unknown resource"}),
                };
                json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":content.to_string()}]}})
            },
            "initialize" => json!({"jsonrpc":"2.0","id":id,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{},"resources":{},"sampling":{}},"serverInfo":{"name":"patch-ts","version":env!("CARGO_PKG_VERSION")}}}),
            _ => json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":format!("Unknown method: {}",method)}}),
        };
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }
    Ok(())
}

fn handle_patch_tool(args: Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let line = args.get("line").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    let fuzz = args.get("fuzz").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let confidence = args.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.9);
    let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
    let force = args.get("force").and_then(|v| v.as_bool()).unwrap_or(false);
    let cli_args = crate::cli::PatchArgs {
        file: Some(file.to_string()), files: None, line: Some(line), fuzz,
        old: Some(old.to_string()), new: Some(new.to_string()), confidence,
        diff: false, delete: None, expect: None, after: None, content: None,
        dry_run, force, no_backup: false, json: false, no_auto_repair: false,
        marker: None, serial: false, plugin: None, allow_all_paths: false,
        url: None, git_commit: None, no_strip_fence: false, no_compile_check: true,
        compile_timeout: 30, no_sanitize: false, no_ellipsis: false,
        uniqueness_weight: 0.2, strict_whitespace: false, cross_file: false,
        agent: None, model: None, no_provenance: true, fix_indent: false,
    };
    let path = PathBuf::from(file);
    match crate::cli::apply_patch_to_file(&path, &cli_args) {
        Ok(()) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Patch applied successfully."}]}}),
        Err(e) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}}),
    }
}

fn handle_balance_tool(args: Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let apply = args.get("apply").and_then(|v| v.as_bool()).unwrap_or(false);
    let max_cost = args.get("max_cost").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let cli_args = crate::cli::BalanceArgs {
        file: Some(file.to_string()), files: None, function: None, apply,
        no_backup: false, max_cost, json: false, serial: false, plugin: None,
        allow_all_paths: false,
    };
    let path = PathBuf::from(file);
    match crate::cli::apply_balance_to_file(&path, &cli_args) {
        Ok(()) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Balance completed."}]}}),
        Err(e) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}}),
    }
}

fn handle_explain_tool(args: Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let line = args.get("line").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
    let mut lang = RustLanguage::new();
    match crate::repair::explain_error(&PathBuf::from(file), line, false, &mut lang) {
        Ok(Some(diag)) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":diag.details}]}}),
        Ok(None) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"No syntax error at this line."}]}}),
        Err(e) => json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}}),
    }
}
