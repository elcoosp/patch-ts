use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

// ... keep all existing tests, we'll only append the JSON test at the end ...


#[test]
fn test_cli_balance_json_actions() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
    let actions = json["actions"].as_array().unwrap();
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0]["type"], "remove");
    assert_eq!(actions[0]["delimiter"], "}");
}
