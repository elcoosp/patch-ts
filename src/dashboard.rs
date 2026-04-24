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
pub fn render_dashboard(state: &DashboardState) -> String {
    let template = include_str!("../assets/dashboard.html");
    let data_json = serde_json::to_string(state).unwrap_or_else(|_| "{}".to_string());

    // Embed data as a script tag before the closing body tag
    let data_script = format!("<script id=\"patch-data\" type=\"application/json\">{}</script>", data_json);

    // Insert the data script before </body>
    template.replace("</body>", &format!("{}\n</body>", data_script))
}
