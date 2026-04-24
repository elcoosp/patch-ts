use patch_ts::watch::FileWatcher;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_create_watcher_and_watch() {
    let dir = tempdir().unwrap();
    let mut watcher = FileWatcher::new(Duration::from_millis(100), vec![], None).unwrap();
    watcher.watch(dir.path()).unwrap();
}

#[test]
fn test_ignore_pattern_compiles() {
    let _watcher =
        FileWatcher::new(Duration::from_millis(100), vec!["*.tmp".to_string()], None).unwrap();
}
