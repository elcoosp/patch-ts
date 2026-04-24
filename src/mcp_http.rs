use anyhow::Result;
use hyper::body::Incoming;
use hyper::body::to_bytes as body_to_bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run_http_mcp(addr: SocketAddr, auth_token: Option<String>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    eprintln!("HTTP MCP server listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let token = auth_token.clone();
        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            let service = service_fn(move |req| handle_request(req, token.clone()));
            if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_request(req: Request<Incoming>, auth_token: Option<String>) -> Result<Response<String>, hyper::Error> {
    // Check authentication
    if let Some(token) = &auth_token {
        let auth_header = req.headers().get("Authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if auth_header != format!("Bearer {}", token) {
            let resp = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("Unauthorized".to_string())
                .unwrap();
            return Ok(resp);
        }
    }

    // Read body using hyper::body::to_bytes (imported)
    let body_bytes = match body_to_bytes(req.into_body()).await {
        Ok(b) => b,
        Err(_) => {
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Failed to read body".to_string())
                .unwrap();
            return Ok(resp);
        }
    };

    let body_str = String::from_utf8_lossy(&body_bytes).to_string();
    let request_json: Value = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => {
            let err = serde_json::json!({"jsonrpc":"2.0","error":{"code":-32700,"message":format!("Parse error: {}",e)},"id":null});
            let resp = Response::new(err.to_string());
            return Ok(resp);
        }
    };

    let method = request_json.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let id = request_json.get("id").cloned().unwrap_or(Value::Null);

    let response = match method {
        "tools/list" => serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {"name":"patch","description":"Apply a patch to a file with tree‑sitter validation","inputSchema":{}},
                    {"name":"balance","description":"Detect and fix unbalanced delimiters","inputSchema":{}}
                ]
            }
        }),
        "initialize" => serde_json::json!({
            "jsonrpc":"2.0","id":id,
            "result":{
                "protocolVersion":"2024-11-05",
                "capabilities":{"tools":{}},
                "serverInfo":{"name":"patch-ts","version":env!("CARGO_PKG_VERSION")}
            }
        }),
        _ => serde_json::json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":format!("Unknown method: {}",method)}}),
    };

    let resp = Response::new(serde_json::to_string(&response).unwrap());
    Ok(resp)
}
