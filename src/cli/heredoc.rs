use anyhow::Result;
pub fn parse_heredoc(input: &str, no_strip_fence: bool, no_sanitize: bool) -> Result<(String, String)> {
    let input = input.trim();
    let content = if !no_sanitize {
        if let Some((_, extracted)) = crate::sanitize::extract_fenced_block(input) { extracted.to_string() }
        else { input.to_string() }
    } else { input.to_string() };
    let content = if !no_strip_fence && content.starts_with("```") && content.ends_with("```") { &content[3..content.len()-3] } else { &content };
    let parts: Vec<&str> = content.split("\n---\n").collect();
    if parts.len() != 2 { anyhow::bail!("Heredoc must contain '<<<' expected block, then '---', then new block"); }
    Ok((parts[0].trim_start_matches("<<<\n").to_string(), parts[1].to_string()))
}
