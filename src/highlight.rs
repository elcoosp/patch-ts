use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});

pub static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| {
    ThemeSet::load_defaults()
});

/// Return the default theme (dark).
pub fn default_theme() -> &'static str {
    "dark"
}

/// Resolve theme name to actual syntect theme.
pub fn resolve_theme_name(name: &str) -> String {
    match name {
        "dark" => "base16-ocean.dark".to_string(),
        "light" => "base16-eighties.light".to_string(),
        "deuteranopia" => "Solarized (light)".to_string(),
        "highcontrast" => "Monokai Extended".to_string(),
        _ => name.to_string(),
    }
}

/// Highlight source code of a given language extension, return an ANSI‑coloured string.
/// Falls back to plain text if language not found.
pub fn highlight(source: &str, extension: &str, theme_name: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(extension)
        .or_else(|| SYNTAX_SET.find_syntax_by_name(extension))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let theme_name = resolve_theme_name(theme_name);
    let theme = THEME_SET.themes.get(&theme_name)
        .unwrap_or_else(|| &THEME_SET.themes["base16-ocean.dark"]);

    // HighlightLines::new directly returns HighlightLines (not Result) in syntect 5.3
    let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
    let ranges: Vec<(syntect::highlighting::Style, &str)> = highlighter.highlight(source, &SYNTAX_SET);
    syntect::util::as_24_bit_terminal_escaped(ranges.as_slice(), false)
}
