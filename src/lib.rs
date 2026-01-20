//! Chatfiles - Minimal text-file-based protocol for multi-agent coordination
//!
//! A simple protocol where agents communicate via shared text files.
//! "If it's stupid but it works, it ain't stupid."

pub mod cli;
pub mod core;
pub mod log;

#[cfg(feature = "web")]
pub mod web;

pub use core::ops::{await_message, join, leave, read, register, send, status};
pub use core::{Chatfile, Error, Result, Session};
