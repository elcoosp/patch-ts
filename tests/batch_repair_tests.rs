use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

// [test]
// fn test_balance_fixes_multiple_errors() {
// let dir = tempdir().unwrap();
// let file_path = dir.path().join("sample.ts");
// fs::write(&file_path, "const x = (1 + 2;\nfunction foo() {\n}\n}\n").unwrap();
//
// let mut cmd = Command::cargo_bin("patch-ts").unwrap();
// cmd.arg("balance")
// .arg("--file").arg(file_path.to_str().unwrap())
// .arg("--apply")
// .assert()
// .success();
//
// let balanced = fs::read_to_string(&file_path).unwrap();
// assert!(balanced.contains("const x = (1 + 2);"));
// assert!(!balanced.contains("}\n}"));
// }

// [test]
//  fn test_balance_fixes_extra_and_missing() {
//      let dir = tempdir().unwrap();
//      let file_path = dir.path().join("sample.js");
//      fs::write(&file_path, "const x = (1 + 2));\nfunction foo() {\n").unwrap();
//
//      let mut cmd = Command::cargo_bin("patch-ts").unwrap();
//      cmd.arg("balance")
//          .arg("--file").arg(file_path.to_str().unwrap())
//          .arg("--apply")
//          .assert()
//          .success();
//
//      let balanced = fs::read_to_string(&file_path).unwrap();
//      assert!(balanced.contains("const x = (1 + 2);"));
//      assert!(balanced.contains("function foo() {\n}"));
//  }
