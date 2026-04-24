use anyhow::Result;
use super::types::*;
pub fn handle_watch(args: WatchArgs) -> Result<()> {
    use crate::watch::FileWatcher;
    use std::time::Duration;
    let ignore_patterns = args.ignore.unwrap_or_default();
    let delay = Duration::from_millis(args.delay);
    let hooks = args.hooks.as_deref();
    let mut watcher = FileWatcher::new(delay, ignore_patterns, hooks)?;
    watcher.watch(&args.path)?;
    let event = watcher.wait_for_change()?;
    println!("Change detected: {:?}", event.paths);
    Ok(())
}
