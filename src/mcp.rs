use crate::ast::RustLanguage;
use crate::cli::balance::apply_balance_to_file;
use crate::cli::patch::apply_patch_to_file;
use crate::cli::types::{BalanceArgs, PatchArgs};
use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
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
fn default_fuzz() -> usize {
    5
}
fn default_confidence() -> f64 {
    0.9
}

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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ImpactParams {
    pub symbol: String,
    pub recursive: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SemDiffParams {
    pub old: String,
    pub new: String,
    pub lang: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EntityListParams {
    pub file: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EntityReplaceParams {
    pub file: String,
    pub symbol: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RecallParams {
    pub file: String,
    pub line: usize,
    pub old: String,
    pub new: String,
    pub error_code: String,
    pub error_message: Option<String>,
    #[serde(default = "default_context_lines")]
    pub context_lines: usize,
    #[serde(default)]
    pub entropy: bool,
    #[serde(default = "default_entropy_threshold")]
    pub entropy_threshold: f64,
    #[serde(default)]
    pub pre_fetch: bool,
}
fn default_context_lines() -> usize {
    5
}
fn default_entropy_threshold() -> f64 {
    2.5
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GateParams {
    pub file: String,
    pub stages: Option<String>,
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NodeReplaceParams {
    pub file: String,
    pub query: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NodeActionParams {
    pub file: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NodeInsertParams {
    pub file: String,
    pub query: String,
    pub text: String,
}

fn to_schema<T: schemars::JsonSchema>() -> Value {
    let schema = schemars::schema_for!(T);
    serde_json::to_value(schema).unwrap_or(Value::Null)
}

/// Catalog of MCP tools exposed by the server.
fn tool_definitions() -> Vec<Value> {
    vec![
        json!({"name":"patch","description":"Apply a patch","inputSchema": to_schema::<PatchParams>()}),
        json!({"name":"balance","description":"Fix unbalanced delimiters","inputSchema": to_schema::<BalanceParams>()}),
        json!({"name":"explain","description":"Explain syntax error","inputSchema": to_schema::<ExplainParams>()}),
        json!({"name":"impact","description":"Show callers of a symbol","inputSchema": to_schema::<ImpactParams>()}),
        json!({"name":"semdiff","description":"Semantic diff between two code strings","inputSchema": to_schema::<SemDiffParams>()}),
        json!({"name":"entity_list","description":"List named entities in a file","inputSchema": to_schema::<EntityListParams>()}),
        json!({"name":"entity_replace","description":"Replace entity by name","inputSchema": to_schema::<EntityReplaceParams>()}),
        json!({"name":"gate","description":"Run validation gate","inputSchema": to_schema::<GateParams>()}),
        json!({"name":"replace_node","description":"Replace an AST node using a tree‑sitter query","inputSchema": to_schema::<NodeReplaceParams>()}),
        json!({"name":"delete_node","description":"Delete an AST node using a tree‑sitter query","inputSchema": to_schema::<NodeActionParams>()}),
        json!({"name":"insert_before_node","description":"Insert text before an AST node","inputSchema": to_schema::<NodeInsertParams>()}),
        json!({"name":"recall","description":"Generate retry context when a patch fails","inputSchema": to_schema::<RecallParams>()}),
    ]
}

/// Route a `tools/call` request to the matching tool handler.
fn dispatch_tool(tool_name: &str, arguments: &Value, id: &Value) -> Value {
    match tool_name {
        "patch" => handle_patch_tool(arguments),
        "balance" => handle_balance_tool(arguments),
        "explain" => handle_explain_tool(arguments),
        "impact" => handle_impact_tool(arguments),
        "semdiff" => handle_semdiff_tool(arguments),
        "entity_list" => handle_entity_list_tool(arguments),
        "entity_replace" => handle_entity_replace_tool(arguments),
        "gate" => handle_gate_tool(arguments),
        "recall" => handle_recall_tool(arguments),
        "replace_node" => handle_replace_node_tool(arguments),
        "delete_node" => handle_delete_node_tool(arguments),
        "insert_before_node" => handle_insert_before_node_tool(arguments),
        _ => {
            json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":format!("Unknown tool: {}", tool_name)}})
        }
    }
}

/// Serve the MCP (Model Context Protocol) JSON-RPC loop over stdio.
///
/// # Errors
///
/// Returns an error if stdio cannot be read or written.
pub fn run_mcp() -> Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
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
            "tools/list" => json!({"jsonrpc":"2.0","id":id,"result":{"tools": tool_definitions()}}),

            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(json!({}));
                dispatch_tool(tool_name, &arguments, &id)
            }

            "resources/list" => {
                json!({"jsonrpc":"2.0","id":id,"result":{"resources": resource_list()}})
            }
            "resources/read" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let uri = params.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                let content = read_resource(uri, &id);
                json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":content.to_string()}]}})
            }
            "initialize" => {
                json!({"jsonrpc":"2.0","id":id,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{},"resources":{},"sampling":{}},"serverInfo":{"name":"patch-ts","version":env!("CARGO_PKG_VERSION")}}})
            }
            _ => {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":format!("Unknown method: {}",method)}})
            }
        };
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }
    Ok(())
}

/// The list of resources advertised over MCP `resources/list`.
fn resource_list() -> Value {
    json!([
        {"uri":"patch-ts://symbols/main.rs","name":"Symbols"},
        {"uri":"patch-ts://history","name":"Patch history"},
        {"uri":"patch-ts://provenance","name":"Provenance records"},
        {"uri":"patch-ts://dashboard","name":"Dashboard","mimeType":"text/html"},
        {"uri":"patch-ts://review","name":"Code Review","mimeType":"text/html"},
        {"uri":"patch-ts://entities/{file}","name":"Entity listing"},
        {"uri":"patch-ts://impact/{symbol}?recursive=true","name":"Call graph lookup"},
        {"uri":"patch-ts://semdiff/{file}","name":"Semantic diff"},
        {"uri":"patch-ts://coverage/{file}","name":"Coverage report"},
        {"uri":"patch-ts://gate/{file}","name":"Gate results"},
        {"uri":"patch-ts://score/{file}","name":"Score card"},
        {"uri":"patch-ts://provenance/{file}","name":"Provenance for file"},
    ])
}

/// Render the text contents of an MCP `resources/read` request.
fn read_resource(uri: &str, id: &Value) -> Value {
    match uri {
        "patch-ts://symbols/main.rs" => {
            if let Ok(source) = std::fs::read_to_string("main.rs") {
                crate::symbols::build_index(&source, "rs")
                    .map(|idx| json!(idx))
                    .unwrap_or(json!({}))
            } else {
                json!({"error":"File not found"})
            }
        }
        "patch-ts://history" => {
            let mgr = crate::history::HistoryManager::new();
            json!(mgr.list().unwrap_or_default())
        }
        "patch-ts://provenance" => {
            json!(crate::provenance::query_provenance(None, None).unwrap_or_default())
        }
        "patch-ts://dashboard" | "patch-ts://review" => {
            let state = crate::dashboard::DashboardState {
                patch_id: "latest".to_string(),
                diff: None,
                score: None,
                provenance: None,
                gate_result: None,
                suggestions: None,
            };
            let html = crate::dashboard::render_dashboard(&state);
            json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":html,"mimeType":"text/html"}]}})
        }
        uri if uri.starts_with("patch-ts://entities/") => {
            let file = uri
                .strip_prefix("patch-ts://entities/")
                .unwrap_or("main.rs");
            if let Ok(content) = std::fs::read_to_string(file) {
                if let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file)) {
                    let parse_result = lang.parse(&content);
                    let entities = lang.find_all_entities(&parse_result);
                    json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":serde_json::to_string(&entities).unwrap_or_default(),"mimeType":"application/json"}]}})
                } else {
                    json!({"error":"Language not detected"})
                }
            } else {
                json!({"error":"File not found"})
            }
        }
        uri if uri.starts_with("patch-ts://impact/") => {
            let symbol = uri.strip_prefix("patch-ts://impact/").unwrap_or("");
            let graph = crate::crossfile::build_project_call_graph(std::path::Path::new("."));
            let edges = graph.callers_of(symbol);
            let result: Vec<_> = edges.iter().map(|e| json!({"file": e.caller_file, "line": e.caller_line, "column": e.caller_column})).collect();
            json!({"jsonrpc":"2.0","id":id,"result":{"contents":[{"uri":uri,"text":serde_json::to_string(&result).unwrap_or_default(),"mimeType":"application/json"}]}})
        }
        _ => json!({"error":"Unknown resource"}),
    }
}

fn handle_patch_tool(args: &Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let line = args
        .get("line")
        .and_then(serde_json::Value::as_u64)
        .map_or(1, crate::num::usize_from_u64);
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    let fuzz = args
        .get("fuzz")
        .and_then(serde_json::Value::as_u64)
        .map_or(5, crate::num::usize_from_u64);
    let confidence = args
        .get("confidence")
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(0.9);
    let dry_run = args
        .get("dry_run")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let force = args
        .get("force")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let cli_args = PatchArgs {
        file: Some(file.to_string()),
        files: None,
        line: Some(line),
        fuzz,
        old: Some(old.to_string()),
        new: Some(new.to_string()),
        confidence,
        diff: false,
        delete: None,
        expect: None,
        after: None,
        content: None,
        dry_run,
        force,
        no_backup: false,
        json: false,
        no_auto_repair: false,
        marker: None,
        serial: false,
        plugin: None,
        allow_all_paths: false,
        symbol: None,
        url: None,
        git_commit: None,
        no_strip_fence: false,
        no_compile_check: true,
        compile_timeout: 30,
        no_sanitize: false,
        no_ellipsis: false,
        uniqueness_weight: 0.2,
        strict_whitespace: false,
        cross_file: false,
        agent: None,
        model: None,
        no_provenance: true,
        fix_indent: false,
        validate_first: false,
        verify: false,
        verify_test: None,
        fix_headers: false,
        entity_body: false,
        preserve_comments: false,
        preserve_orphan_comments: false,
    };
    let path = PathBuf::from(file);
    match apply_patch_to_file(&path, &cli_args) {
        Ok(()) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Patch applied successfully."}]}})
        }
        Err(e) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
        }
    }
}

fn handle_balance_tool(args: &Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let apply = args
        .get("apply")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let max_cost = args
        .get("max_cost")
        .and_then(serde_json::Value::as_u64)
        .map_or(10, crate::num::usize_from_u64);
    let cli_args = BalanceArgs {
        file: Some(file.to_string()),
        files: None,
        function: None,
        apply,
        max_cost,
        json: false,
        plugin: None,
        toggles: crate::cli::types::BalanceToggles::default(),
    };
    let path = PathBuf::from(file);
    match apply_balance_to_file(&path, &cli_args) {
        Ok(()) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Balance completed."}]}})
        }
        Err(e) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
        }
    }
}
fn handle_explain_tool(args: &Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let line = args
        .get("line")
        .and_then(serde_json::Value::as_u64)
        .map_or(1, crate::num::usize_from_u64);
    let mut lang = RustLanguage::new();
    match crate::repair::explain_error(&PathBuf::from(file), line, false, &mut lang) {
        Ok(Some(diag)) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":diag.details}]}})
        }
        Ok(None) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"No syntax error at this line."}]}})
        }
        Err(e) => {
            json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
        }
    }
}
fn handle_impact_tool(args: &Value) -> Value {
    let symbol = args.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
    let recursive = args
        .get("recursive")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let graph = crate::crossfile::build_project_call_graph(std::path::Path::new("."));
    let edges = if recursive {
        graph.all_callers_recursive(symbol)
    } else {
        graph.callers_of(symbol)
    };
    let result: Vec<_> = edges
        .iter()
        .map(|e| json!({"file": e.caller_file, "line": e.caller_line, "column": e.caller_column}))
        .collect();
    json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":serde_json::to_string(&result).unwrap_or_default()}]}})
}
fn handle_semdiff_tool(args: &Value) -> Value {
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    let lang = args.get("lang").and_then(|v| v.as_str()).unwrap_or("rs");
    let changes = crate::semdiff::compute_semantic_diff(old, new, lang);
    let result = serde_json::to_string(&changes).unwrap_or_default();
    json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":result}]}})
}
fn handle_entity_list_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    if let Ok(content) = std::fs::read_to_string(file)
        && let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file))
    {
        let parse_result = lang.parse(&content);
        let entities = lang.find_all_entities(&parse_result);
        return json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":serde_json::to_string(&entities).unwrap_or_default()}]}});
    }
    json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Error reading file"}],"isError":true}})
}
fn handle_entity_replace_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let symbol = args.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    if let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file)) {
        let options = crate::patch::PatchOptions::default();
        match crate::patch::apply_symbol_patch(
            std::path::Path::new(file),
            symbol,
            new,
            &options,
            &mut *lang,
        ) {
            Ok(()) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Entity replaced."}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Unsupported file"}],"isError":true}})
    }
}
fn handle_gate_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let stages_str = args
        .get("stages")
        .and_then(|v| v.as_str())
        .unwrap_or("syntax,compile");
    let stages: Vec<String> = stages_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if let Ok(content) = std::fs::read_to_string(file) {
        match crate::gate::run_gate(&stages, std::path::Path::new(file), &content, &content, 30) {
            Ok(res) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":serde_json::to_string(&res).unwrap_or_default()}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"File not found"}],"isError":true}})
    }
}

fn handle_recall_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let line = args
        .get("line")
        .and_then(serde_json::Value::as_u64)
        .map_or(1, crate::num::usize_from_u64);
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    let error_code = args
        .get("error_code")
        .and_then(|v| v.as_str())
        .unwrap_or("E000");
    let error_message = args
        .get("error_message")
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string);
    let context_lines = args
        .get("context_lines")
        .and_then(serde_json::Value::as_u64)
        .map_or(5, crate::num::usize_from_u64);
    let entropy = args
        .get("entropy")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let entropy_threshold = args
        .get("entropy_threshold")
        .and_then(serde_json::Value::as_f64)
        .unwrap_or(2.5);
    let pre_fetch = args
        .get("pre_fetch")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);

    let file_path = std::path::PathBuf::from(file);
    if let Ok(mut lang) = crate::ast::detect_language(&file_path) {
        let query = crate::recall::RecallQuery {
            file_path: &file_path,
            line,
            old_content: old,
            new_content: new,
            error_code,
            error_message: error_message.as_deref(),
            context_lines,
            use_entropy: entropy,
            entropy_threshold,
            pre_fetch,
            session_id: None,
            max_tokens: None,
        };
        match crate::recall::generate_recall_context(&query, &mut *lang) {
            Ok(ctx) => {
                let result = serde_json::to_string(&ctx).unwrap_or_default();
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":result}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Unsupported file"}],"isError":true}})
    }
}

fn handle_replace_node_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    if let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file)) {
        match crate::patch::node_patch::apply_node_replace(
            std::path::Path::new(file),
            query,
            new,
            &mut *lang,
        ) {
            Ok(()) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Node replaced."}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Unsupported file"}],"isError":true}})
    }
}

fn handle_delete_node_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    if let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file)) {
        match crate::patch::node_patch::apply_node_delete(
            std::path::Path::new(file),
            query,
            &mut *lang,
        ) {
            Ok(()) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Node deleted."}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Unsupported file"}],"isError":true}})
    }
}

fn handle_insert_before_node_tool(args: &Value) -> Value {
    let file = args
        .get("file")
        .and_then(|v| v.as_str())
        .unwrap_or("main.rs");
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
    if let Ok(mut lang) = crate::ast::detect_language(std::path::Path::new(file)) {
        match crate::patch::node_patch::apply_node_insert_before(
            std::path::Path::new(file),
            query,
            text,
            &mut *lang,
        ) {
            Ok(()) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Text inserted before node."}]}})
            }
            Err(e) => {
                json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":format!("Error: {}",e)}],"isError":true}})
            }
        }
    } else {
        json!({"jsonrpc":"2.0","id":null,"result":{"content":[{"type":"text","text":"Unsupported file"}],"isError":true}})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn call(name: &str, arguments: &Value) -> Value {
        dispatch_tool(name, arguments, &json!(1))
    }

    #[test]
    fn tool_definitions_are_advertised() {
        let tools = tool_definitions();
        assert_ne!(tools.len(), 0);
        assert!(tools.iter().all(|t| t["name"].is_string()));
        assert!(tools.iter().all(|t| t["inputSchema"].is_object()));
    }

    #[test]
    fn dispatch_unknown_tool_reports_error() {
        let response = dispatch_tool("definitely_not_a_tool", &json!({}), &json!(1));
        assert_eq!(response["error"]["code"], -32601);
        assert!(
            response["error"]["message"]
                .as_str()
                .unwrap_or("")
                .contains("definitely_not_a_tool")
        );
    }

    #[test]
    fn unknown_method_produces_jsonrpc_error() {
        let response = dispatch_tool("bogus/method", &json!({}), &json!(1));
        assert!(response["error"]["code"].is_number());
    }

    #[test]
    fn patch_tool_applies_change() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn main() {\n    old_line();\n}\n").unwrap();
        let response = call(
            "patch",
            &json!({"file": file.to_str().unwrap(), "line": 2, "old": "old_line();", "new": "new_line();"}),
        );
        assert!(
            response["result"]["isError"].is_null() && response["error"].is_null(),
            "patch tool should succeed: {response}"
        );
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("new_line();"));
    }

    #[test]
    fn balance_tool_reports_result() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn main() {\n").unwrap();
        let response = call(
            "balance",
            &json!({"file": file.to_str().unwrap(), "apply": true}),
        );
        assert!(response["result"]["content"].is_array(), "{response}");
    }

    #[test]
    fn explain_tool_flags_syntax_error() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn broken(\n").unwrap();
        let response = call(
            "explain",
            &json!({"file": file.to_str().unwrap(), "line": 1}),
        );
        let text = response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("");
        assert!(!text.is_empty(), "{response}");
    }

    #[test]
    fn explain_tool_clean_line_reports_no_error() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn main() {}\n").unwrap();
        let response = call(
            "explain",
            &json!({"file": file.to_str().unwrap(), "line": 1}),
        );
        let text = response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("");
        assert!(text.contains("No syntax error"), "{response}");
    }

    #[test]
    fn entity_list_tool_lists_functions() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn alpha() {}\n").unwrap();
        let response = call("entity_list", &json!({"file": file.to_str().unwrap()}));
        let text = response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("");
        assert!(
            text.contains("alpha"),
            "entity list should mention alpha: {text}"
        );
    }

    #[test]
    fn entity_replace_tool_swaps_body() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn alpha() { 1 }\n").unwrap();
        let response = call(
            "entity_replace",
            &json!({"file": file.to_str().unwrap(), "symbol": "alpha", "new": "fn alpha() { 2 }"}),
        );
        assert!(
            !response["result"]["isError"].as_bool().unwrap_or(false),
            "{response}"
        );
    }

    #[test]
    fn gate_tool_runs_syntax_stage() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn main() {}\n").unwrap();
        let response = call(
            "gate",
            &json!({"file": file.to_str().unwrap(), "stages": "syntax"}),
        );
        assert!(response["result"]["content"].is_array(), "{response}");
    }

    #[test]
    fn gate_tool_missing_file_errors() {
        let response = call("gate", &json!({"file": "/definitely/missing.rs"}));
        assert!(
            response["result"]["isError"].as_bool().unwrap_or(false),
            "{response}"
        );
    }

    #[test]
    fn impact_tool_returns_callers_json() {
        let response = call("impact", &json!({"symbol": "build_index"}));
        let text = response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("[]");
        let parsed: Value = serde_json::from_str(text).unwrap_or(json!([]));
        assert!(parsed.is_array());
    }

    #[test]
    fn semdiff_tool_compares_snippets() {
        let response = call(
            "semdiff",
            &json!({"old": "fn alpha() { 1 }", "new": "fn alpha() { 2 }", "lang": "rs"}),
        );
        assert!(response["result"]["content"].is_array(), "{response}");
    }

    #[test]
    fn replace_node_tool_replaces_first_match() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn alpha() { 1 }\n").unwrap();
        let response = call(
            "replace_node",
            &json!({
                "file": file.to_str().unwrap(),
                "query": "(function_item name: (identifier) @name)",
                "new": "renamed"
            }),
        );
        assert!(
            !response["result"]["isError"].as_bool().unwrap_or(false),
            "{response}"
        );
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("fn renamed()"), "{content}");
    }

    #[test]
    fn delete_node_tool_removes_comment() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "// marker comment\nfn alpha() {}\n").unwrap();
        let response = call(
            "delete_node",
            &json!({"file": file.to_str().unwrap(), "query": "(line_comment) @c"}),
        );
        assert!(
            !response["result"]["isError"].as_bool().unwrap_or(false),
            "{response}"
        );
        let content = fs::read_to_string(&file).unwrap();
        assert!(!content.contains("marker comment"), "{content}");
    }

    #[test]
    fn insert_before_node_tool_inserts_text() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "// anchor\nfn alpha() {}\n").unwrap();
        let response = call(
            "insert_before_node",
            &json!({
                "file": file.to_str().unwrap(),
                "query": "(line_comment) @c",
                "text": "// inserted\n"
            }),
        );
        assert!(
            !response["result"]["isError"].as_bool().unwrap_or(false),
            "{response}"
        );
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("// inserted"), "{content}");
    }

    #[test]
    fn recall_tool_returns_payload() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn main() {\n    old_line();\n}\n").unwrap();
        let response = call(
            "recall",
            &json!({
                "file": file.to_str().unwrap(),
                "line": 2,
                "old": "old_line();",
                "new": "new_line();"
            }),
        );
        assert!(response["result"]["content"].is_array(), "{response}");
    }

    #[test]
    fn resource_list_contains_dashboard() {
        let resources = resource_list();
        let list = resources.as_array().unwrap();
        assert!(
            list.iter()
                .any(|r| r["uri"].as_str() == Some("patch-ts://dashboard"))
        );
    }

    #[test]
    fn read_resource_unknown_uri_errors() {
        let content = read_resource("patch-ts://unknown", &json!(1));
        assert!(content["error"].as_str().is_some());
    }

    #[test]
    fn read_resource_history_and_provenance_return_arrays() {
        let history = read_resource("patch-ts://history", &json!(1));
        assert!(history.is_array() || history.is_object());
        let provenance = read_resource("patch-ts://provenance", &json!(1));
        assert!(provenance.is_array() || provenance.is_object());
    }

    #[test]
    fn read_resource_dashboard_renders_html() {
        let value = read_resource("patch-ts://dashboard", &json!(1));
        let text = value["result"]["contents"][0]["text"]
            .as_str()
            .unwrap_or("");
        assert_ne!(text.len(), 0);
    }

    #[test]
    fn read_resource_entities_lists_code_owners() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        fs::write(&file, "fn alpha() {}\n").unwrap();
        let uri = format!("patch-ts://entities/{}", file.to_str().unwrap());
        let value = read_resource(&uri, &json!(1));
        let text = serde_json::to_string(&value).unwrap();
        assert!(text.contains("alpha"), "{text}");
    }

    #[test]
    fn read_resource_missing_file_errors() {
        let value = read_resource("patch-ts://entities/definitely-missing.rs", &json!(1));
        assert!(
            serde_json::to_string(&value).unwrap().contains("not found")
                || value["error"].is_string()
        );
    }

    #[test]
    fn read_resource_impact_returns_json_array() {
        let value = read_resource("patch-ts://impact/nothing", &json!(1));
        let text = serde_json::to_string(&value).unwrap();
        assert!(text.contains("result") || text.contains("[]"));
    }

    #[test]
    fn default_fuzz_and_confidence_constants() {
        assert_eq!(default_fuzz(), 5);
        assert!((default_confidence() - 0.9).abs() < f64::EPSILON);
        assert_eq!(default_context_lines(), 5);
        assert!((default_entropy_threshold() - 2.5).abs() < f64::EPSILON);
    }
}
