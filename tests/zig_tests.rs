use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_zig_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("main.zig");
    fs::write(&file_path, "const std = @import(\"std\");\npub fn main() void {\n    std.debug.print(\"hello\", .{});\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("    std.debug.print(\"hello\", .{});")
        .arg("--new").arg("    std.debug.print(\"world\", .{});")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("std.debug.print(\"world\", .{});"));
}
