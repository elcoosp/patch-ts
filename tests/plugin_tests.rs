use assert_cmd::Command;
use patch_ts::plugin::PluginHost;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_plugin_host_loads_and_calls() {
    let plugin_path = Path::new("tests/fixtures/sample_plugin.wasm");
    let host = PluginHost::load(plugin_path).unwrap();

    use patch_ts::ast::{DelimiterError, Span};
    let errors = vec![DelimiterError::Extra {
        span: Span {
            start_byte: 6,
            end_byte: 7,
            start_line: 1,
            start_column: 7,
            end_line: 1,
            end_column: 8,
        },
        delimiter: '}',
    }];
    let source = "fn foo() {} }";

    let repaired = host.repair(&errors, source).unwrap();
    assert_eq!(repaired, "fn foo() {} }");
}

#[test]
fn test_cli_balance_with_plugin() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--plugin")
        .arg("tests/fixtures/sample_plugin.wasm")
        .arg("--apply")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "fn main() {}");
}
