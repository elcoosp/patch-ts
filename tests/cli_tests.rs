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

#[test]
fn test_balance_max_cost_exceeded() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("broken.rs");
    // File with many errors, e.g., 20 extra closing braces
    let content = "fn main() {}".to_string() + &" }".repeat(20);
    fs::write(&file_path, content).unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--max-cost").arg("3")
        .arg("--json")
        .assert()
        .success()  // CLI exits 0, failure is in JSON body
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"]["message"].as_str().unwrap().contains("max cost"));
}

#[test]
fn test_patch_json_confidence_anchor() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "// comment\nhello world\n").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("hello world")
        .arg("--new").arg("hello patch-ts")
        .arg("--json")
        .arg("--fuzz").arg("2")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
    assert!(json["confidence"].as_f64().unwrap() > 0.9);
    assert_eq!(json["strategy"], "anchor");
}

#[test]
fn test_fix_indent_spaces_to_tabs() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    // File uses tabs
    fs::write(&file_path, "\t\tlet x = 1;\n").unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("let x = 1;")
        .arg("--new").arg("let x = 2;")
        .arg("--fix-indent")
        .arg("--fuzz").arg("5")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    // The new line should have the same tab indentation
    assert!(content.contains("\t\tlet x = 2;"));
}
