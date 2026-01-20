use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

use chrono::Local;
use notify::{Event, RecursiveMode, Watcher};

use super::error::{Error, Result};
use crate::log;

#[derive(Debug)]
pub struct Chatfile {
    pub path: PathBuf,
}

impl Chatfile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            log::warn("Chatfile", &format!("File not found: {}", path.display()));
            return Err(Error::ChatfileNotFound(path));
        }
        log::debug("Chatfile", &format!("Opened: {}", path.display()));
        Ok(Self { path })
    }

    pub fn create(name: Option<&str>) -> Result<Self> {
        let filename = match name {
            Some(n) => format!("{n}.Chatfile"),
            None => "Chatfile".to_string(),
        };
        let path = PathBuf::from(&filename);

        if path.exists() {
            log::warn(
                "Chatfile",
                &format!("Room already exists: {}", path.display()),
            );
            return Err(Error::RoomExists(path));
        }

        let room_name = name.unwrap_or("default");
        let timestamp = Local::now().format("%F %T");
        let header = format!(
            "[system {timestamp}]: Chatroom \"{room_name}\". Format: Name: msg. Append only.\n"
        );

        std::fs::write(&path, header)?;
        Self::try_set_append_only(&path);

        log::info("Chatfile", &format!("Created room: {}", path.display()));
        Ok(Self { path })
    }

    #[cfg(target_os = "linux")]
    fn try_set_append_only(path: &Path) {
        let _ = std::process::Command::new("sudo")
            .args(["chattr", "+a"])
            .arg(path)
            .status();
    }

    #[cfg(not(target_os = "linux"))]
    fn try_set_append_only(_path: &Path) {}

    pub fn list_rooms() -> Result<Vec<PathBuf>> {
        let mut rooms = Vec::new();

        for entry in std::fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name == "Chatfile" || name.ends_with(".Chatfile") {
                    rooms.push(path);
                }
            }
        }

        rooms.sort();
        Ok(rooms)
    }

    pub fn append(&self, content: &str) -> Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.path)?;
        writeln!(file, "{content}")?;
        Ok(())
    }

    pub fn send(&self, name: &str, message: &str) -> Result<()> {
        if message.is_empty() {
            log::warn("Chatfile", "Attempted to send empty message");
            return Err(Error::EmptyMessage);
        }
        log::debug("Chatfile", &format!("{name} sending message"));
        self.append(&format!("{name}: {message}"))
    }

    pub fn announce_join(&self, name: &str) -> Result<()> {
        self.append(&format!("[{name} joined]"))
    }

    pub fn announce_leave(&self, name: &str) -> Result<()> {
        self.append(&format!("[{name} left]"))
    }

    pub fn read_last(&self, n: usize) -> Result<Vec<String>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<std::io::Result<_>>()?;

        let start = lines.len().saturating_sub(n);
        Ok(lines[start..].to_vec())
    }

    pub fn last_line(&self) -> Result<Option<String>> {
        let lines = self.read_last(1)?;
        Ok(lines.into_iter().next())
    }

    /// Extracts sender name from a message line.
    /// Returns None for system messages (starting with `[`) or lines without sender.
    pub fn get_sender(line: &str) -> Option<&str> {
        // System messages like [user joined] or [system ...] are not from users
        if line.starts_with('[') {
            return None;
        }
        // Must have format "name: message"
        let colon_pos = line.find(':')?;
        if colon_pos == 0 {
            return None;
        }
        Some(&line[..colon_pos])
    }

    pub fn name_exists(&self, name: &str) -> Result<bool> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let prefix = format!("{name}:");

        for line in reader.lines() {
            if line?.starts_with(&prefix) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn watch(&self) -> Result<String> {
        // Get initial line count
        let initial_lines = self.count_lines()?;

        // Set up file watcher with channel
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res: std::result::Result<Event, _>| {
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    let _ = tx.send(());
                }
            }
        })
        .map_err(|e| Error::Io(std::io::Error::other(format!("watcher error: {e}"))))?;

        watcher
            .watch(&self.path, RecursiveMode::NonRecursive)
            .map_err(|e| Error::Io(std::io::Error::other(format!("watch error: {e}"))))?;

        log::debug("Chatfile", "Waiting for new message (inotify)...");

        // Wait for file modification events
        loop {
            // Block until we get a notification (with timeout to prevent infinite hang)
            match rx.recv_timeout(Duration::from_secs(60)) {
                Ok(()) => {
                    // File was modified, check for new lines
                    let current_lines = self.count_lines()?;
                    if current_lines > initial_lines {
                        // Read the last line
                        if let Some(line) = self.last_line()? {
                            return Ok(line);
                        }
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout - just continue waiting
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(Error::Io(std::io::Error::other("watcher disconnected")));
                }
            }
        }
    }

    fn count_lines(&self) -> Result<usize> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines().count())
    }
}
