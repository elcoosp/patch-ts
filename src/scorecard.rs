use crate::score::ReliabilityScore;
use std::fmt::Write as _;

/// Render a colour‑coded terminal scorecard.
/// Returns a String with ANSI escape codes.
#[must_use]
pub fn render_scorecard(score: &ReliabilityScore) -> String {
    let mut out = String::new();
    out.push_str("\n\u{1f4ca} Reliability Scorecard\n\n");
    let _ = writeln!(out, "Overall: {}/100", score.overall);
    out.push_str(&bar("Overall        ", score.overall));
    out.push('\n');

    for (dim, val) in &score.dimensions {
        out.push_str(&bar(&format!("{dim:16}"), *val));
        out.push('\n');
    }
    out.push('\n');
    out
}

fn bar(label: &str, value: u8) -> String {
    let filled = (value as usize) * 20 / 100;
    let empty = 20 - filled;
    let color = match value {
        0..=39 => "\x1b[31m",  // red
        40..=69 => "\x1b[33m", // yellow
        _ => "\x1b[32m",       // green
    };
    format!(
        "{} {} {}{}{}\x1b[0m",
        label,
        value,
        color,
        "█".repeat(filled),
        "░".repeat(empty)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn scorecard_renders_overall_and_dimensions() {
        let mut dimensions = HashMap::new();
        dimensions.insert("syntax".to_string(), 90u8);
        dimensions.insert("security".to_string(), 70u8);
        let score = ReliabilityScore {
            overall: 88,
            dimensions,
        };
        let rendered = render_scorecard(&score);
        assert!(rendered.contains("88"));
        assert!(rendered.contains("syntax"));
        assert!(rendered.contains("security"));
    }
}
