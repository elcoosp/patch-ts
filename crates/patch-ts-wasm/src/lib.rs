use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PatchRequest {
    pub file_path: String,
    pub line: usize,
    pub old: String,
    pub new: String,
    pub fuzz: Option<usize>,
}

#[derive(Serialize)]
pub struct PatchResponse {
    pub success: bool,
    pub content: Option<String>,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub fn apply_patch_wasm(request_json: &str) -> String {
    let _request: PatchRequest = match serde_json::from_str(request_json) {
        Ok(r) => r,
        Err(e) => {
            let resp = PatchResponse { success: false, content: None, error: Some(format!("Invalid JSON: {}", e)) };
            return serde_json::to_string(&resp).unwrap_or_default();
        }
    };
    serde_json::to_string(&PatchResponse {
        success: true,
        content: Some("WASM patch applied (stub)".to_string()),
        error: None,
    }).unwrap_or_default()
}
