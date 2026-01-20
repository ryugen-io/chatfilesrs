use crate::core::{self, Chatfile};

pub fn create_room(name: Option<&str>) -> i32 {
    match Chatfile::create(name) {
        Ok(cf) => {
            println!("Created room: {}", cf.path.display());
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn list_rooms() -> i32 {
    match Chatfile::list_rooms() {
        Ok(rooms) => {
            println!("Available rooms:");
            for room in rooms {
                println!("  {}", room.display());
            }
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn register(chatfile: &str, name: Option<&str>) -> i32 {
    match core::ops::register(chatfile, name) {
        Ok(session) => {
            println!("{}", session.name);
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn join() -> i32 {
    match core::ops::join() {
        Ok(session) => {
            println!("Joined as {}", session.name);
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn leave() -> i32 {
    match core::ops::leave() {
        Ok(_) => {
            println!("Left room");
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn send(message: &str) -> i32 {
    match core::ops::send(message) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn admin_send(message: &str) -> i32 {
    match core::ops::admin_send(message) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn await_message() -> i32 {
    match core::ops::await_message() {
        Ok(msg) => {
            println!("{msg}");
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn send_await(message: &str) -> i32 {
    if send(message) != 0 {
        return 1;
    }
    await_message()
}

pub fn read(n: usize) -> i32 {
    match core::ops::read(n) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn status() -> i32 {
    match core::ops::status() {
        Ok(session) => {
            println!("Session: {}", session.name);
            println!("Chatfile: {}", session.chatfile.display());
            println!("Joined: {}", if session.joined { "yes" } else { "no" });
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

pub fn clear(force: bool, sessions_only: bool) -> i32 {
    let files = match core::ops::list_clearable_files(sessions_only) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    if files.is_empty() {
        println!("No files to clear.");
        return 0;
    }

    println!("The following files will be PERMANENTLY DELETED:");
    println!("{}", core::ops::format_file_list(&files));

    if !force {
        use std::io::Write;
        print!("\nAre you sure you want to proceed? [y/N] ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return 0;
        }
    }

    let result = core::ops::clear_files(&files);

    for path in &result.removed {
        println!("[OK] Removed: {}", path.display());
    }

    for (path, err) in &result.failed {
        eprintln!("[ERROR] {}: {}", path.display(), err);
    }

    if result.failed.is_empty() {
        println!("\nCleared {} files.", result.removed.len());
        0
    } else {
        eprintln!(
            "\nPartially cleared: {} removed, {} failed.",
            result.removed.len(),
            result.failed.len()
        );
        1
    }
}

#[cfg(feature = "web")]
pub fn serve(port: u16, dir: &str) -> i32 {
    match crate::web::serve(port, dir) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}
