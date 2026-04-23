use anyhow::{Context, Result};
use glob::Pattern;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<notify::Event>>,
    watched_paths: Vec<PathBuf>,
    debounce_delay: Duration,
    ignore_patterns: Vec<Pattern>,
    hook_command: Option<String>,
    last_event_time: Instant,
}

impl FileWatcher {
    pub fn new(
        debounce_delay: Duration,
        ignore_patterns: Vec<String>,
        hook_command: Option<&str>,
    ) -> Result<Self> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(500)),
        )?;
        let patterns: Vec<Pattern> = ignore_patterns
            .iter()
            .map(|p| Pattern::new(p).unwrap())
            .collect();
        Ok(Self {
            watcher,
            rx,
            watched_paths: Vec::new(),
            debounce_delay,
            ignore_patterns: patterns,
            hook_command: hook_command.map(|s| s.to_string()),
            last_event_time: Instant::now(),
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

    pub fn wait_for_change(&mut self) -> Result<notify::Event> {
        loop {
            match self.rx.recv() {
                Ok(Ok(event)) => {
                    if event.kind.is_modify() && !event.paths.is_empty() {
                        let now = Instant::now();
                        if now - self.last_event_time < self.debounce_delay {
                            continue;
                        }
                        self.last_event_time = now;

                        let should_ignore = event.paths.iter().any(|p| {
                            self.ignore_patterns.iter().any(|pat| pat.matches_path(p))
                        });
                        if should_ignore {
                            continue;
                        }

                        if let Some(ref hook) = self.hook_command {
                            let file = event.paths.first()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let cmd = hook.replace("%file%", &file);
                            let status = ProcessCommand::new("sh")
                                .arg("-c")
                                .arg(&cmd)
                                .status()
                                .with_context(|| format!("Failed to run hook: {}", cmd))?;
                            if !status.success() {
                                eprintln!("Warning: hook exited with {}", status);
                            }
                        }
                        return Ok(event);
                    }
                }
                Ok(Err(e)) => anyhow::bail!("Watch error: {}", e),
                Err(e) => anyhow::bail!("Channel error: {}", e),
            }
        }
    }
}
