use std::path::PathBuf;
use std::process::Command;

use directories::ProjectDirs;

use super::error::Result;

#[derive(Debug, Default)]
pub struct ClearableFiles {
    pub chatfiles: Vec<PathBuf>,
    pub sessions: Vec<PathBuf>,
    pub admin: Option<PathBuf>,
}

impl ClearableFiles {
    pub fn is_empty(&self) -> bool {
        self.chatfiles.is_empty() && self.sessions.is_empty() && self.admin.is_none()
    }

    pub fn total_count(&self) -> usize {
        self.chatfiles.len() + self.sessions.len() + if self.admin.is_some() { 1 } else { 0 }
    }
}

#[derive(Debug, Default)]
pub struct ClearResult {
    pub removed: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
}

pub fn list_clearable_files(sessions_only: bool) -> Result<ClearableFiles> {
    let mut files = ClearableFiles::default();

    // Check CWD for chatfiles and legacy sessions
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if !sessions_only && (name == "Chatfile" || name.ends_with(".Chatfile")) {
                files.chatfiles.push(path);
            } else if name.starts_with(".cf_session") {
                // Legacy session files in CWD
                files.sessions.push(path);
            } else if name == ".cf_admin" {
                files.admin = Some(path);
            }
        }
    }

    // Check XDG sessions directory
    if let Some(proj_dirs) = ProjectDirs::from("", "", "chatfiles") {
        let sessions_dir = proj_dirs.data_dir().join("sessions");
        if sessions_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&sessions_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "session") {
                        files.sessions.push(path);
                    }
                }
            }
        }
    }

    // Check home directory for legacy session
    if let Some(base_dirs) = directories::BaseDirs::new() {
        let home_session = base_dirs.home_dir().join(".cf_session");
        if home_session.exists() && !files.sessions.contains(&home_session) {
            files.sessions.push(home_session);
        }
    }

    files.chatfiles.sort();
    files.sessions.sort();

    Ok(files)
}

pub fn clear_files(files: &ClearableFiles) -> ClearResult {
    let mut result = ClearResult::default();

    for path in &files.chatfiles {
        if let Err(e) = remove_file_with_chattr(path) {
            result.failed.push((path.clone(), e));
        } else {
            result.removed.push(path.clone());
        }
    }

    for path in &files.sessions {
        if let Err(e) = std::fs::remove_file(path) {
            result.failed.push((path.clone(), e.to_string()));
        } else {
            result.removed.push(path.clone());
        }
    }

    if let Some(ref path) = files.admin {
        if let Err(e) = std::fs::remove_file(path) {
            result.failed.push((path.clone(), e.to_string()));
        } else {
            result.removed.push(path.clone());
        }
    }

    result
}

fn remove_file_with_chattr(path: &PathBuf) -> std::result::Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("sudo")
            .args(["chattr", "-a"])
            .arg(path)
            .status();
    }

    std::fs::remove_file(path).map_err(|e| e.to_string())
}

pub fn format_file_list(files: &ClearableFiles) -> String {
    let mut lines = Vec::new();

    if !files.chatfiles.is_empty() {
        lines.push("Chatfiles:".to_string());
        for f in &files.chatfiles {
            lines.push(format!("  {}", f.display()));
        }
    }

    if !files.sessions.is_empty() {
        lines.push("Session files:".to_string());
        for f in &files.sessions {
            lines.push(format!("  {}", f.display()));
        }
    }

    if let Some(ref f) = files.admin {
        lines.push(format!("Admin file:\n  {}", f.display()));
    }

    lines.join("\n")
}
