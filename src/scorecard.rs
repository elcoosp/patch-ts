use crate::score::ReliabilityScore;

/// Render a colour‑coded terminal scorecard.
/// Returns a String with ANSI escape codes.
pub fn render_scorecard(score: &ReliabilityScore) -> String {
    let mut out = String::new();
    out.push_str("\n\u{1f4ca} Reliability Scorecard\n\n");
    out.push_str(&format!("Overall: {}/100\n", score.overall));
    out.push_str(&bar("Overall        ", score.overall));
    out.push_str("\n");

    for (dim, val) in &score.dimensions {
        out.push_str(&bar(&format!("{:16}", dim), *val));
        out.push('\n');
    }
    out.push('\n');
    out
}

fn bar(label: &str, value: u8) -> String {
    let filled = (value as usize) * 20 / 100;
    let empty = 20 - filled;
    let color = match value {
        0..=39 => "\x1b[31m", // red
        40..=69 => "\x1b[33m", // yellow
        _ => "\x1b[32m", // green
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
