use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub static SYNTAX_SET: std::sync::LazyLock<SyntaxSet> =
    std::sync::LazyLock::new(SyntaxSet::load_defaults_newlines);

pub static THEME_SET: std::sync::LazyLock<ThemeSet> =
    std::sync::LazyLock::new(ThemeSet::load_defaults);

#[must_use]
pub fn default_theme() -> &'static str {
    "dark"
}

#[must_use]
pub fn resolve_theme_name(name: &str) -> String {
    match name {
        "dark" => "base16-ocean.dark".to_string(),
        "light" => "base16-eighties.light".to_string(),
        "deuteranopia" => "Solarized (light)".to_string(),
        "highcontrast" => "Monokai Extended".to_string(),
        _ => name.to_string(),
    }
}

pub fn highlight(source: &str, extension: &str, theme_name: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(extension)
        .or_else(|| SYNTAX_SET.find_syntax_by_name(extension))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let theme_name_str = resolve_theme_name(theme_name);
    let theme = THEME_SET
        .themes
        .get(&theme_name_str)
        .unwrap_or_else(|| &THEME_SET.themes["base16-ocean.dark"]);

    let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
    // Highlight line by line and accumulate terminal escapes
    let mut output = String::new();
    for line in source.lines() {
        if let Ok(ranges) = highlighter.highlight_line(line, &SYNTAX_SET) {
            output.push_str(&syntect::util::as_24_bit_terminal_escaped(
                &ranges[..],
                false,
            ));
            output.push('\n');
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_is_dark() {
        assert_eq!(default_theme(), "dark");
    }

    #[test]
    fn resolve_theme_name_maps_known_names() {
        assert_eq!(resolve_theme_name("dark"), "base16-ocean.dark");
        assert_eq!(resolve_theme_name("light"), "base16-eighties.light");
        assert_eq!(resolve_theme_name("deuteranopia"), "Solarized (light)");
        assert_eq!(resolve_theme_name("highcontrast"), "Monokai Extended");
        assert_eq!(resolve_theme_name("custom-theme"), "custom-theme");
    }

    #[test]
    fn highlight_produces_ansi_for_known_language() {
        let out = highlight("fn main() {}\n", "rs", "dark");
        assert!(out.contains('\u{1b}'), "expected ANSI escapes");
    }

    #[test]
    fn highlight_falls_back_to_plain_text_for_unknown_extension() {
        let out = highlight("just text\n", "definitely-not-a-lang", "dark");
        assert!(out.contains("just text"));
    }
}
