use rand::Rng;

use super::chatfile::Chatfile;
use super::error::{Error, Result};

const ADJECTIVES: &[&str] = &[
    "swift", "bold", "calm", "keen", "sage", "wild", "bright", "dark", "quick", "slow",
];

const NOUNS: &[&str] = &[
    "fox", "owl", "raven", "wolf", "bear", "hawk", "crane", "lynx", "deer", "hare",
];

const MAX_ATTEMPTS: u32 = 100;

pub fn generate(chatfile: &Chatfile) -> Result<String> {
    let mut rng = rand::rng();

    for _ in 0..MAX_ATTEMPTS {
        let adj = ADJECTIVES[rng.random_range(0..ADJECTIVES.len())];
        let noun = NOUNS[rng.random_range(0..NOUNS.len())];
        let suffix: u32 = rng.random_range(1000..10000);

        let name = format!("{adj}-{noun}-{suffix}");

        if !chatfile.name_exists(&name)? {
            return Ok(name);
        }
    }

    Err(Error::NameGenerationFailed(MAX_ATTEMPTS))
}

pub fn validate(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(Error::InvalidName("Name cannot be empty".into()));
    }
    if name.len() > 32 {
        return Err(Error::InvalidName("Name is too long (max 32 chars)".into()));
    }
    if name.contains(':') {
        return Err(Error::InvalidName("Name cannot contain colons".into()));
    }
    if name.contains('[') || name.contains(']') {
        return Err(Error::InvalidName("Name cannot contain brackets".into()));
    }
    if name.contains('\n') || name.contains('\r') {
        return Err(Error::InvalidName("Name cannot contain newlines".into()));
    }
    Ok(())
}

pub fn resolve_custom(name: &str, chatfile: &Chatfile) -> Result<String> {
    validate(name)?;

    // First try: exact match
    if !chatfile.name_exists(name)? {
        return Ok(name.to_string());
    }

    // Subsequent tries: append _N
    for i in 2..100 {
        let candidate = format!("{}_{}", name, i);
        if !chatfile.name_exists(&candidate)? {
            return Ok(candidate);
        }
    }

    Err(Error::NameGenerationFailed(100))
}
