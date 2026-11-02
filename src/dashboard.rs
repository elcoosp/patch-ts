use serde::Serialize;

#[derive(Serialize)]
pub struct DashboardState {
    pub patch_id: String,
    pub diff: Option<String>,
    pub score: Option<serde_json::Value>,
    pub provenance: Option<Vec<serde_json::Value>>,
    pub gate_result: Option<serde_json::Value>,
    pub suggestions: Option<Vec<String>>,
}

/// Render the MCP Apps dashboard HTML for a given patch state.
#[must_use]
pub fn render_dashboard(state: &DashboardState) -> String {
    let template = include_str!("../assets/dashboard.html");
    let data_json = serde_json::to_string(state).unwrap_or_else(|_| "{}".to_string());
    let data_script =
        format!("<script id=\"patch-data\" type=\"application/json\">{data_json}</script>");
    template.replace("</body>", &format!("{data_script}\n</body>"))
}
