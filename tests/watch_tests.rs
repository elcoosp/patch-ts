use patch_ts::watch::FileWatcher;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_create_watcher_and_watch() {
    let dir = tempdir().unwrap();
    // Just verify the watcher can be created and watches a directory
    let mut watcher = FileWatcher::new(
        Duration::from_millis(100),
        vec![],
        None,
    )
    .unwrap();
    watcher.watch(dir.path()).unwrap();
    // If we reach here, the watcher is set up correctly.
}

#[test]
fn test_ignore_pattern_compiles() {
    // Verify that patterns don't cause a panic
    let _watcher = FileWatcher::new(
        Duration::from_millis(100),
        vec!["*.tmp".to_string()],
        None,
    )
    .unwrap();
    // Pattern compilation succeeded
}
