use directories::ProjectDirs;
use std::path::PathBuf;

use super::error::{Error, Result};

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("io", "ryugen", "chatfiles").ok_or(Error::XdgError)
}

pub fn data_dir() -> Result<PathBuf> {
    Ok(project_dirs()?.data_dir().to_path_buf())
}

pub fn config_dir() -> Result<PathBuf> {
    Ok(project_dirs()?.config_dir().to_path_buf())
}

pub fn sessions_dir() -> Result<PathBuf> {
    Ok(data_dir()?.join("sessions"))
}
