use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use chrono::Local;

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

    pub fn get_sender(line: &str) -> Option<&str> {
        line.split(':').next()
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
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::End(0))?;

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(0) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Ok(_) => {
                    return Ok(line.trim_end().to_string());
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
}
