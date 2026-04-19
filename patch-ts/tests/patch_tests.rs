use patch_ts::patch::{apply_literal_patch, delete_line, insert_lines, apply_unified_diff, PatchOptions};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_replace_block_exact_match() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let options = PatchOptions::default();
    apply_literal_patch(&file_path, 2, "line2", "new line2", options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "line1\nnew line2\nline3\n");
}

#[test]
fn test_replace_block_mismatch_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let options = PatchOptions::default();
    let result = apply_literal_patch(&file_path, 2, "wrong line", "new line2", options);
    assert!(result.is_err());
    // The error message mentions similarity threshold, not exact expected/actual
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("no match found with similarity"));
}

#[test]
fn test_replace_block_with_fuzz() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "// added comment\nline1\nline2\nline3\n").unwrap();

    let options = PatchOptions {
        fuzz_radius: 3,
        similarity_threshold: 0.9,
        ..Default::default()
    };
    apply_literal_patch(&file_path, 2, "line2", "new line2", options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "// added comment\nline1\nnew line2\nline3\n");
}

#[test]
fn test_fuzzy_match_ambiguous() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line A\nline B\nline A\n").unwrap();

    let options = PatchOptions {
        fuzz_radius: 3,
        similarity_threshold: 0.9,
        ..Default::default()
    };
    // Should succeed (picks first exact match)
    apply_literal_patch(&file_path, 2, "line A", "new line", options).unwrap();
}

#[test]
fn test_delete_line_with_expect() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let options = PatchOptions::default();
    delete_line(&file_path, 2, "line2", options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "line1\nline3\n");
}

#[test]
fn test_delete_line_mismatch_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let options = PatchOptions::default();
    let result = delete_line(&file_path, 2, "wrong", options);
    assert!(result.is_err());
}

#[test]
fn test_insert_lines() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline4\n").unwrap();

    let options = PatchOptions::default();
    insert_lines(&file_path, 2, "line3", options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "line1\nline2\nline3\nline4\n");
}

#[test]
fn test_apply_unified_diff() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let diff = r#"--- a/sample.rs
+++ b/sample.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let options = PatchOptions::default();
    apply_unified_diff(&file_path, diff, options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "line1\nnew line2\nline3\n");
}

#[test]
fn test_apply_diff_with_fuzz() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    // Content has an extra comment line, shifting the target lines
    fs::write(&file_path, "// new comment\nline1\nline2\nline3\n").unwrap();

    let diff = r#"--- a/sample.rs
+++ b/sample.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let options = PatchOptions {
        fuzz_radius: 3,
        ..Default::default()
    };
    apply_unified_diff(&file_path, diff, options).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "// new comment\nline1\nnew line2\nline3\n");
}
