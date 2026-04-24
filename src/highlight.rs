use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});

pub static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| {
    ThemeSet::load_defaults()
});

pub fn default_theme() -> &'static str {
    "dark"
}

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
    let theme = THEME_SET.themes.get(&theme_name_str)
        .unwrap_or_else(|| &THEME_SET.themes["base16-ocean.dark"]);

    let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
    // Highlight line by line and accumulate terminal escapes
    let mut output = String::new();
    for line in source.lines() {
        match highlighter.highlight_line(line, &SYNTAX_SET) {
            Ok(ranges) => {
                output.push_str(&syntect::util::as_24_bit_terminal_escaped(&ranges[..], false));
                output.push('\n');
            }
            Err(_) => {
                output.push_str(line);
                output.push('\n');
            }
        }
    }
    output
}
