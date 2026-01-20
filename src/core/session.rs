use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use super::error::{Error, Result};
use crate::log;

#[derive(Debug, Clone)]
pub struct Session {
    pub chatfile: PathBuf,
    pub name: String,
    pub joined: bool,
}

impl Session {
    pub fn new(chatfile: PathBuf, name: String) -> Self {
        Self {
            chatfile,
            name,
            joined: false,
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::find_session_file()?;
        log::debug("Session", &format!("Loading from: {}", path.display()));
        Self::load_from(&path)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut lines = content.lines();

        let chatfile = lines.next().ok_or(Error::InvalidSession)?.trim().into();

        let name = lines
            .next()
            .ok_or(Error::InvalidSession)?
            .trim()
            .to_string();

        let joined = lines.next().is_some_and(|s| s.trim() == "yes");

        Ok(Self {
            chatfile,
            name,
            joined,
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::session_path_for(&self.chatfile)?;
        log::debug("Session", &format!("Saving to: {}", path.display()));
        self.save_to(&path)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let joined_str = if self.joined { "yes" } else { "" };
        let content = format!(
            "{}\n{}\n{}\n",
            self.chatfile.display(),
            self.name,
            joined_str
        );
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Returns the XDG data directory for chatfiles sessions.
    /// ~/.local/share/chatfiles/sessions/
    fn sessions_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "chatfiles").ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine XDG directories",
            ))
        })?;
        Ok(proj_dirs.data_dir().join("sessions"))
    }

    /// Generates a session filename based on the chatfile path hash.
    fn session_filename(chatfile: &Path) -> String {
        let canonical = std::fs::canonicalize(chatfile).unwrap_or_else(|_| chatfile.to_path_buf());

        let mut hasher = DefaultHasher::new();
        canonical.to_string_lossy().hash(&mut hasher);
        let hash = hasher.finish();

        format!("{:016x}.session", hash)
    }

    /// Returns the session file path for a given chatfile.
    fn session_path_for(chatfile: &Path) -> Result<PathBuf> {
        // CF_SESSION env var takes precedence
        if let Ok(env_path) = std::env::var("CF_SESSION") {
            return Ok(PathBuf::from(env_path));
        }

        let sessions_dir = Self::sessions_dir()?;
        let filename = Self::session_filename(chatfile);
        Ok(sessions_dir.join(filename))
    }

    fn find_session_file() -> Result<PathBuf> {
        // CF_SESSION env var takes precedence
        if let Ok(env_path) = std::env::var("CF_SESSION") {
            let path = PathBuf::from(&env_path);
            if path.exists() {
                return Ok(path);
            }
            return Err(Error::NoSession);
        }

        // Check XDG sessions directory for any session file
        if let Ok(sessions_dir) = Self::sessions_dir() {
            if sessions_dir.exists() {
                // Find the most recently modified session file
                if let Ok(entries) = std::fs::read_dir(&sessions_dir) {
                    let mut sessions: Vec<_> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "session"))
                        .collect();

                    sessions.sort_by_key(|e| {
                        e.metadata()
                            .and_then(|m| m.modified())
                            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                    });

                    if let Some(latest) = sessions.last() {
                        return Ok(latest.path());
                    }
                }
            }
        }

        // Legacy: check CWD for .cf_session
        let local = PathBuf::from(".cf_session");
        if local.exists() {
            log::debug("Session", "Found legacy .cf_session in CWD");
            return Ok(local);
        }

        // Legacy: check home directory
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let home_session = base_dirs.home_dir().join(".cf_session");
            if home_session.exists() {
                log::debug("Session", "Found legacy .cf_session in home");
                return Ok(home_session);
            }
        }

        Err(Error::NoSession)
    }

    /// Lists all active sessions in the XDG sessions directory.
    pub fn list_sessions() -> Result<Vec<PathBuf>> {
        let sessions_dir = Self::sessions_dir()?;
        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }

        let sessions = std::fs::read_dir(&sessions_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "session"))
            .map(|e| e.path())
            .collect();

        Ok(sessions)
    }

    /// Deletes the session file for the current chatfile.
    pub fn delete(&self) -> Result<()> {
        let path = Self::session_path_for(&self.chatfile)?;
        if path.exists() {
            std::fs::remove_file(&path)?;
            log::debug("Session", &format!("Deleted: {}", path.display()));
        }
        Ok(())
    }
}
