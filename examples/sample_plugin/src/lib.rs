// No need for serde; we use the generated types directly.

wit_bindgen::generate!({
    world: "plugin",
    inline: r#"
        package patch-ts:plugin;

        interface repair {
            record span {
                start-byte: u32,
                end-byte: u32,
            }

            variant delimiter-error {
                extra(tuple<char, span>),
                missing(tuple<char, span>),
            }

            repair: func(errors: list<delimiter-error>, source: string) -> string;
        }

        world plugin {
            export repair;
        }
    "#,
});

use exports::patch_ts::plugin::repair::{DelimiterError, Guest, Span};

struct MyPlugin;

impl Guest for MyPlugin {
    fn repair(errors: Vec<DelimiterError>, mut source: String) -> String {
        for err in errors {
            match err {
                DelimiterError::Extra((_ch, span)) => {
                    let start = span.start_byte as usize;
                    let end = span.end_byte as usize;
                    if start < source.len() && end <= source.len() {
                        source.replace_range(start..end, "");
                    }
                }
                DelimiterError::Missing((_ch, _span)) => {
                    // For simplicity, we ignore missing delimiters in this sample.
                }
            }
        }
        source
    }
}

export!(MyPlugin);
