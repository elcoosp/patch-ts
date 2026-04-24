use termimad::*;
use crate::gate::StageResult;

/// Render a gate report in Markdown and return the string.
pub fn render_gate_report_md(stages: &[StageResult], passed: bool) -> String {
    let mut md = String::new();
    md.push_str("## 🔧 patch‑ts Gate Report\n\n");
    md.push_str("| Stage | Status | Details |\n");
    md.push_str("|-------|--------|---------|\n");
    for stage in stages {
        let icon = if stage.passed { "✅" } else { "❌" };
        md.push_str(&format!("| {} | {} | {} |\n", stage.name, icon, stage.details));
    }
    md.push_str(&format!("\n**Overall:** {}\n", if passed { "✅ Passed" } else { "❌ Failed" }));
    md
}

/// Print the Markdown report to terminal using termimad, falling back to plain text.
pub fn print_md(md: &str) {
    let skin = MadSkin::default();  // MadSkin::default returns MadSkin directly
    if terminal_size().is_some() {
        skin.print_text(md);
    } else {
        println!("{}", md);
    }
}

fn terminal_size() -> Option<(u16, u16)> {
    Some(termimad::terminal_size())  // termimad::terminal_size returns (u16, u16)
}
