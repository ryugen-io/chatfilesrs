use std::path::PathBuf;

use super::chatfile::Chatfile;
use super::error::{Error, Result};

const ADMIN_FILE: &str = ".cf_admin";
const DEFAULT_ADMIN_PREFIX: &str = "admin";

#[derive(Debug, Clone)]
pub struct AdminConfig {
    pub prefix: String,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            prefix: DEFAULT_ADMIN_PREFIX.to_string(),
        }
    }
}

impl AdminConfig {
    pub fn load() -> Result<Self> {
        let path = find_admin_file()?;
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let prefix = content
            .lines()
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        Ok(Self {
            prefix: prefix.unwrap_or(DEFAULT_ADMIN_PREFIX).to_string(),
        })
    }
}

pub fn is_admin() -> bool {
    find_admin_file().is_ok()
}

pub fn verify_admin() -> Result<AdminConfig> {
    if !is_admin() {
        return Err(Error::NotAdmin);
    }
    AdminConfig::load()
}

pub fn admin_send(chatfile_path: &str, message: &str) -> Result<()> {
    let config = verify_admin()?;

    if message.is_empty() {
        return Err(Error::EmptyMessage);
    }

    let chatfile = Chatfile::open(chatfile_path)?;
    chatfile.append(&format!("[{}]: {}", config.prefix, message))
}

fn find_admin_file() -> Result<PathBuf> {
    // Check CWD first (project-local admin)
    let local = PathBuf::from(ADMIN_FILE);
    if local.exists() {
        return Ok(local);
    }

    // Check XDG config directory
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "chatfiles") {
        let xdg_admin = proj_dirs.config_dir().join(ADMIN_FILE);
        if xdg_admin.exists() {
            return Ok(xdg_admin);
        }
    }

    // Legacy: check home directory
    if let Some(home) = directories::BaseDirs::new() {
        let home_admin = home.home_dir().join(ADMIN_FILE);
        if home_admin.exists() {
            return Ok(home_admin);
        }
    }

    Err(Error::NotAdmin)
}
