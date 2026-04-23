use anyhow::Result;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;

/// Run a bare‑bones MCP (JSON‑RPC 2.0) server on stdin/stdout.
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
                let err = json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) },
                    "id": null
                });
                writeln!(stdout, "{}", serde_json::to_string(&err)?)?;
                stdout.flush()?;
                continue;
            }
        };

        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = request.get("id").cloned().unwrap_or(Value::Null);

        let response = match method {
            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "patch",
                            "description": "Apply a patch to a file with tree‑sitter validation and fuzzy matching",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "file": { "type": "string" },
                                    "line": { "type": "integer" },
                                    "old": { "type": "string" },
                                    "new": { "type": "string" },
                                    "fuzz": { "type": "integer", "default": 5 },
                                    "confidence": { "type": "number", "default": 0.9 },
                                    "dry_run": { "type": "boolean" },
                                    "force": { "type": "boolean" }
                                },
                                "required": ["file", "line", "old", "new"]
                            }
                        },
                        {
                            "name": "balance",
                            "description": "Detect and fix unbalanced delimiters",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "file": { "type": "string" },
                                    "apply": { "type": "boolean", "default": false },
                                    "max_cost": { "type": "integer", "default": 10 }
                                },
                                "required": ["file"]
                            }
                        },
                        {
                            "name": "explain",
                            "description": "Explain syntax error at a given line",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "file": { "type": "string" },
                                    "line": { "type": "integer" }
                                },
                                "required": ["file", "line"]
                            }
                        }
                    ]
                }
            }),

            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or(Value::Null);
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                match tool_name {
                    "patch" => handle_patch_tool(args),
                    "balance" => handle_balance_tool(args),
                    "explain" => handle_explain_tool(args),
                    _ => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": { "code": -32601, "message": format!("Unknown tool: {}", tool_name) }
                    }),
                }
            }

            "initialize" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "patch-ts",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            }),

            _ => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("Unknown method: {}", method) }
            }),
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
        url: None,
        git_commit: None,
        no_strip_fence: false,
        fix_indent: false,
        no_compile_check: true,  // MCP doesn't run compilation by default
        compile_timeout: 30,
    };

    let path = PathBuf::from(file);
    match crate::cli::apply_patch_to_file(&path, &cli_args) {
        Ok(()) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": "Patch applied successfully." }] }
        }),
        Err(e) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": format!("Error: {}", e) }], "isError": true }
        }),
    }
}

fn handle_balance_tool(args: Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let apply = args.get("apply").and_then(|v| v.as_bool()).unwrap_or(false);
    let max_cost = args.get("max_cost").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let cli_args = crate::cli::BalanceArgs {
        file: Some(file.to_string()),
        files: None,
        function: None,
        apply,
        no_backup: false,
        max_cost,
        json: false,
        serial: false,
        plugin: None,
        allow_all_paths: false,
    };

    let path = PathBuf::from(file);
    match crate::cli::apply_balance_to_file(&path, &cli_args) {
        Ok(()) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": "Balance completed." }] }
        }),
        Err(e) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": format!("Error: {}", e) }], "isError": true }
        }),
    }
}

fn handle_explain_tool(args: Value) -> Value {
    let file = args.get("file").and_then(|v| v.as_str()).unwrap_or("");
    let line = args.get("line").and_then(|v| v.as_u64()).unwrap_or(1) as usize;

    let mut lang = match crate::cli::detect_language(&PathBuf::from(file)) {
        Ok(l) => l,
        Err(e) => return json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": format!("Error: {}", e) }], "isError": true }
        }),
    };

    match crate::repair::explain_error(&PathBuf::from(file), line, false, &mut *lang) {
        Ok(Some(diag)) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": diag.details }] }
        }),
        Ok(None) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": "No syntax error at this line." }] }
        }),
        Err(e) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "result": { "content": [{ "type": "text", "text": format!("Error: {}", e) }], "isError": true }
        }),
    }
}
