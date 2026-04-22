use anyhow::{Context, Result};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<notify::Event>>,
    watched_paths: Vec<PathBuf>,
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(500)),
        )?;
        Ok(Self {
            watcher,
            rx,
            watched_paths: Vec::new(),
        })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        self.watcher
            .watch(path, RecursiveMode::NonRecursive)
            .with_context(|| format!("Failed to watch path: {}", path.display()))?;
        self.watched_paths.push(path.to_path_buf());
        Ok(())
    }

    pub fn wait_for_change(&self) -> Result<notify::Event> {
        loop {
            match self.rx.recv() {
                Ok(Ok(event)) => {
                    if event.kind.is_modify() && !event.paths.is_empty() {
                        return Ok(event);
                    }
                }
                Ok(Err(e)) => anyhow::bail!("Watch error: {}", e),
                Err(e) => anyhow::bail!("Channel error: {}", e),
            }
        }
    }
}
