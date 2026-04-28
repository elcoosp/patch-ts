use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_zig_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("main.zig");
    fs::write(&fp, "const std = @import(\"std\");\npub fn main() void {\n    std.debug.print(\"hello\", .{});\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\n    std.debug.print(\"hello\", .{});\n---\n    std.debug.print(\"world\", .{});\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("std.debug.print(\"world\", .{});"));
}
