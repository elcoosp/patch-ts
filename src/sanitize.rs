use anyhow::Result;
use regex::Regex;

/// Strip <think> blocks (and <thinking>, </thinking> variants) from LLM output.
pub fn strip_think_blocks(input: &str) -> String {
    let re = Regex::new(r"(?s)<\s*(?:think|thinking)\s*>.*?</\s*(?:think|thinking)\s*>").unwrap();
    re.replace_all(input, "").into_owned()
}

/// Strip surrounding prose, leaving only JSON-like content.
pub fn extract_json_block(input: &str) -> &str {
    let start = input.find('{').unwrap_or(0);
    let end = input.rfind('}').map(|i| i + 1).unwrap_or(input.len());
    &input[start..end]
}

/// Repair malformed JSON using jsonrepair, then parse with serde_json.
pub fn repair_json(input: &str) -> Result<serde_json::Value> {
    let repaired = jsonrepair::repair_json(input, &jsonrepair::Options::default())
        .map_err(|e| anyhow::anyhow!("JSON repair failed: {}", e))?;
    let value = serde_json::from_str(&repaired)
        .map_err(|e| anyhow::anyhow!("JSON parse after repair: {}", e))?;
    Ok(value)
}

/// Full sanitization pipeline for LLM‑produced JSON.
pub fn sanitize_json(input: &str) -> Result<serde_json::Value> {
    let cleaned = strip_think_blocks(input);
    let json_block = extract_json_block(&cleaned);
    repair_json(json_block)
}

/// Extract any ```fenced``` block from mixed‑content LLM output.
/// Returns (language_tag, content).
pub fn extract_fenced_block(input: &str) -> Option<(&str, &str)> {
    let re = Regex::new(r"(?s)```(\w*)\n(.*?)```").unwrap();
    let caps = re.captures(input)?;
    Some((caps.get(1)?.as_str(), caps.get(2)?.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_think_blocks() {
        let input = "<think>reasoning</think>{\"a\": 1}";
        let cleaned = strip_think_blocks(input);
        assert_eq!(cleaned.trim(), "{\"a\": 1}");
    }

    #[test]
    fn test_repair_trailing_comma() {
        let input = "{\"a\": 1,}";
        let value = repair_json(input).unwrap();
        assert_eq!(value["a"], 1);
    }

    #[test]
    fn test_repair_single_quotes() {
        let input = "{'a': 'hello'}";
        let value = repair_json(input).unwrap();
        assert_eq!(value["a"], "hello");
    }

    #[test]
    fn test_extract_json_from_prose() {
        let input = "Sure! Here's the result: {\"status\": \"ok\"} Hope that helps!";
        let cleaned = extract_json_block(input);
        assert_eq!(cleaned, "{\"status\": \"ok\"}");
    }

    #[test]
    fn test_extract_fenced_block() {
        let input = "Here:\n```diff\n--- a/b\n+++ a/b\n@@ -1 +1 @@\n-old\n+new\n```";
        let (tag, content) = extract_fenced_block(input).unwrap();
        assert_eq!(tag, "diff");
        assert!(content.contains("old"));
    }
}
