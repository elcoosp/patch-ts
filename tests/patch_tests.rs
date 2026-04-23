use patch_ts::ast::RustLanguage;
use patch_ts::patch::{
    apply_literal_patch, apply_unified_diff, delete_line, insert_lines, PatchOptions,
};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_replace_block_exact_match() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions::default();
    apply_literal_patch(&file_path, 2, "line2", "new line2", &mut options, &mut lang).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "line1\nnew line2\nline3\n");
}

#[test]
fn test_replace_block_mismatch_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions {
        fuzz_radius: 0,
        similarity_threshold: 1.0,
        ..Default::default()
    };
    let result = apply_literal_patch(&file_path, 2, "wrong line", "new line2", &mut options, &mut lang);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("expected line 2 to contain"));
}

#[test]
fn test_replace_block_with_fuzz() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "// added comment\nline1\nline2\nline3\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions {
        fuzz_radius: 3,
        similarity_threshold: 0.9,
        ..Default::default()
    };
    apply_literal_patch(&file_path, 2, "line2", "new line2", &mut options, &mut lang).unwrap();

    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "// added comment\nline1\nnew line2\nline3\n");
}

#[test]
fn test_fuzzy_match_ambiguous() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line A\nline B\nline A\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions {
        fuzz_radius: 3,
        similarity_threshold: 0.9,
        ..Default::default()
    };
    // The cascade now returns the first match, so it succeeds.
    let result = apply_literal_patch(&file_path, 2, "line A", "new line", &mut options, &mut lang);
    assert!(result.is_ok());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("new line"));
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

#[test]
fn test_fuzzy_match_below_threshold_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nlineX\nline3\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions {
        fuzz_radius: 2,
        similarity_threshold: 0.99,
        ..Default::default()
    };
    let result = apply_literal_patch(&file_path, 2, "line2", "new line", &mut options, &mut lang);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("no match found with any strategy"));
}

#[test]
fn test_empty_search_range_fails() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions::default();
    let result = apply_literal_patch(&file_path, 1, "line", "new", &mut options, &mut lang);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("empty") || err.contains("out of range"));
}

#[test]
fn test_apply_unified_diff_malformed() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "completely different content\n").unwrap();

    let diff = r#"--- a/sample.rs
+++ b/sample.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let options = PatchOptions::default();
    let result = apply_unified_diff(&file_path, diff, options);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Failed to apply diff"));
}

#[test]
fn test_delete_line_out_of_range() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\n").unwrap();

    let options = PatchOptions::default();
    let result = delete_line(&file_path, 5, "line2", options);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("out of range"));
}

#[test]
fn test_insert_lines_out_of_range() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\n").unwrap();

    let options = PatchOptions::default();
    let result = insert_lines(&file_path, 5, "line2", options);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("out of range"));
}

#[test]
fn test_apply_literal_patch_out_of_range() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\n").unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions::default();
    let result = apply_literal_patch(&file_path, 100, "line1", "new", &mut options, &mut lang);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("out of range"));
}

#[test]
fn test_delete_line_mismatch_message() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let options = PatchOptions::default();
    let result = delete_line(&file_path, 2, "wrong", options);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("expected line 2 to contain 'wrong'"));
}

#[test]
fn test_apply_unified_diff_error() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "content\n").unwrap();

    let diff = r#"--- a/sample.rs
+++ b/sample.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let options = PatchOptions::default();
    let result = apply_unified_diff(&file_path, diff, options);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to apply diff"));
}
