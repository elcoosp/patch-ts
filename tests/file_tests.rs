use std::path::Path;
use patch_ts::file::FileManager;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_atomic_write_preserves_content() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    let original = "fn main() {}\n";
    fs::write(&file_path, original).unwrap();

    let manager = FileManager::new(false);
    let new_content = "fn main() { println!(\"hello\"); }\n";
    manager.write_atomic(&file_path, new_content).unwrap();

    let read_back = fs::read_to_string(&file_path).unwrap();
    assert_eq!(read_back, new_content);
}

#[test]
fn test_backup_created() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    let original = "fn main() {}\n";
    fs::write(&file_path, original).unwrap();

    let manager = FileManager::new(true);
    let new_content = "fn main() { println!(\"hello\"); }\n";
    manager.write_atomic(&file_path, new_content).unwrap();

    let backup_path = dir.path().join("test.rs.bak");
    assert!(backup_path.exists());
    let backup_content = fs::read_to_string(&backup_path).unwrap();
    assert_eq!(backup_content, original);
}

#[test]
fn test_no_backup_when_disabled() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    let original = "fn main() {}\n";
    fs::write(&file_path, original).unwrap();

    let manager = FileManager::new(false);
    let new_content = "fn main() { println!(\"hello\"); }\n";
    manager.write_atomic(&file_path, new_content).unwrap();

    let backup_path = dir.path().join("test.rs.bak");
    assert!(!backup_path.exists());
}

#[test]
fn test_read_missing_file() {
    let manager = FileManager::new(false);
    let result = manager.read(Path::new("/definitely/not/a/real/file.rs"));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to read file"));
}
