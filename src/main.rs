use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::from(chatfiles::cli::run() as u8)
}
