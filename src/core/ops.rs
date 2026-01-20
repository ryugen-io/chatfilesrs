use std::path::Path;

use super::admin;
use super::chatfile::Chatfile;
use super::clear;
use super::error::{Error, Result};
use super::names;
use super::session::Session;
use crate::log;

pub fn register(chatfile_path: impl AsRef<Path>, custom_name: Option<&str>) -> Result<Session> {
    log::debug(
        "ops",
        &format!("Registering chatfile: {:?}", chatfile_path.as_ref()),
    );
    let path = chatfile_path.as_ref();
    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    let chatfile = Chatfile::open(&canonical)?;

    let name = match custom_name {
        Some(n) => {
            log::debug("ops", &format!("Resolving custom name: {}", n));
            names::resolve_custom(n, &chatfile)?
        }
        None => {
            log::debug("ops", "Generating random name");
            names::generate(&chatfile)?
        }
    };

    let session = Session::new(canonical, name.clone());
    session.save()?;

    log::info(
        "ops",
        &format!("Registered session '{}' for {:?}", name, path),
    );
    Ok(session)
}

pub fn join() -> Result<Session> {
    let mut session = Session::load()?;

    if session.joined {
        log::warn("ops", &format!("Already joined as {}", session.name));
        return Err(Error::AlreadyJoined(session.name));
    }

    let chatfile = Chatfile::open(&session.chatfile)?;
    chatfile.announce_join(&session.name)?;

    session.joined = true;
    session.save()?;

    log::info("ops", &format!("Joined room as {}", session.name));
    Ok(session)
}

pub fn leave() -> Result<Session> {
    let mut session = Session::load()?;

    if !session.joined {
        log::warn("ops", "Attempted to leave but not joined");
        return Err(Error::NotJoined);
    }

    let chatfile = Chatfile::open(&session.chatfile)?;
    chatfile.announce_leave(&session.name)?;

    session.joined = false;
    session.save()?;

    log::info("ops", &format!("Left room: {}", session.name));
    Ok(session)
}

pub fn send(message: &str) -> Result<()> {
    let session = Session::load()?;

    if !session.joined {
        return Err(Error::NotJoined);
    }

    let chatfile = Chatfile::open(&session.chatfile)?;
    log::debug("ops", &format!("Sending message: '{}'", message));
    chatfile.send(&session.name, message)
}

pub fn admin_send(message: &str) -> Result<()> {
    let session = Session::load()?;
    let chatfile_path = session.chatfile.to_string_lossy();
    log::info("ops", &format!("Admin sending: '{}'", message));
    admin::admin_send(&chatfile_path, message)
}

pub fn await_message() -> Result<String> {
    let session = Session::load()?;

    if !session.joined {
        return Err(Error::NotJoined);
    }

    let chatfile = Chatfile::open(&session.chatfile)?;

    if let Some(last) = chatfile.last_line()? {
        if let Some(sender) = Chatfile::get_sender(&last) {
            if sender != session.name {
                return Ok(last);
            }
        }
    }

    chatfile.watch()
}

pub fn read(n: usize) -> Result<Vec<String>> {
    let session = Session::load()?;
    let chatfile = Chatfile::open(&session.chatfile)?;
    chatfile.read_last(n)
}

pub fn status() -> Result<Session> {
    Session::load()
}

pub fn list_clearable_files(sessions_only: bool) -> Result<clear::ClearableFiles> {
    clear::list_clearable_files(sessions_only)
}

pub fn format_file_list(files: &clear::ClearableFiles) -> String {
    clear::format_file_list(files)
}

pub fn clear_files(files: &clear::ClearableFiles) -> clear::ClearResult {
    clear::clear_files(files)
}
