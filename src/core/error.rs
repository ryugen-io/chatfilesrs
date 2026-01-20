use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No session found. Run: cf register <chatfile>")]
    NoSession,

    #[error("Chatfile not found: {0}")]
    ChatfileNotFound(PathBuf),

    #[error("Room already exists: {0}")]
    RoomExists(PathBuf),

    #[error("Already joined as {0}")]
    AlreadyJoined(String),

    #[error("Not in room. Run: cf join")]
    NotJoined,

    #[error("Message cannot be empty")]
    EmptyMessage,

    #[error("Failed to generate unique name after {0} attempts")]
    NameGenerationFailed(u32),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid session file format")]
    InvalidSession,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Admin privileges required. Create .cf_admin file to enable.")]
    NotAdmin,

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Failed to determine XDG base directories")]
    XdgError,
}

pub type Result<T> = std::result::Result<T, Error>;
