use crate::gate::StageResult;
use std::fmt::Write as _;
use termimad::MadSkin;

/// Render a gate report in Markdown and return the string.
#[must_use]
pub fn render_gate_report_md(stages: &[StageResult], passed: bool) -> String {
    let mut md = String::new();
    md.push_str("## 🔧 patch‑ts Gate Report\n\n");
    md.push_str("| Stage | Status | Details |\n");
    md.push_str("|-------|--------|---------|\n");
    for stage in stages {
        let icon = if stage.passed { "✅" } else { "❌" };
        let _ = writeln!(md, "| {} | {} | {} |", stage.name, icon, stage.details);
    }
    let _ = writeln!(
        md,
        "\n**Overall:** {}",
        if passed { "✅ Passed" } else { "❌ Failed" }
    );
    md
}

/// Print the Markdown report to terminal using termimad, falling back to plain text.
pub fn print_md(md: &str) {
    let skin = MadSkin::default(); // MadSkin::default returns MadSkin directly
    if termimad::terminal_size() == (0, 0) {
        println!("{md}");
    } else {
        skin.print_text(md);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::StageResult;

    #[test]
    fn gate_report_md_lists_stages_and_status() {
        let stages = vec![
            StageResult {
                name: "syntax".to_string(),
                passed: true,
                details: String::new(),
            },
            StageResult {
                name: "compile".to_string(),
                passed: false,
                details: "boom".to_string(),
            },
        ];
        let md = render_gate_report_md(&stages, false);
        assert!(md.contains("syntax"));
        assert!(md.contains("compile"));
        assert!(md.contains("boom"));
        assert!(md.contains("fail") || md.contains("❌") || md.contains("FAIL"));
    }

    #[test]
    fn print_md_writes_without_panicking() {
        print_md("# hello");
    }
}
