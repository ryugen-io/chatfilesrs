pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cf")]
#[command(about = "Chatfile tool for multi-agent coordination")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new room (name.Chatfile), append-only
    #[command(visible_aliases = ["create", "cr"])]
    CreateRoom {
        /// Room name (creates name.Chatfile, or Chatfile if omitted)
        name: Option<String>,
    },

    /// List available rooms
    #[command(visible_aliases = ["list", "ls"])]
    ListRooms,

    /// Register with a chatfile
    #[command(visible_aliases = ["reg", "r"])]
    Register {
        /// Path to chatfile (default: Chatfile)
        #[arg(default_value = "Chatfile")]
        chatfile: String,

        /// Custom display name (default: random name)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Join the room (announces entry)
    #[command(visible_alias = "j")]
    Join,

    /// Leave the room (announces exit)
    #[command(visible_alias = "l")]
    Leave,

    /// Send a message
    #[command(visible_alias = "s")]
    Send {
        /// Message to send
        message: String,
    },

    /// Send a message as admin
    #[command(visible_aliases = ["as", "admin"])]
    AdminSend {
        /// Message to send
        message: String,
    },

    /// Wait for next message
    #[command(visible_aliases = ["a", "wait", "w"])]
    Await,

    /// Send and wait for reply
    #[command(visible_alias = "sa")]
    SendAwait {
        /// Message to send
        message: String,
    },

    /// Show last n messages (default 20)
    #[command(visible_alias = "cat")]
    Read {
        /// Number of messages to show
        #[arg(default_value = "20")]
        n: usize,
    },

    /// Show current session
    #[command(visible_alias = "st")]
    Status,

    /// Clear chatfiles and session data
    #[command(visible_aliases = ["cls", "clean"])]
    Clear {
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,

        /// Only delete session files, keep Chatfiles
        #[arg(short, long)]
        sessions_only: bool,
    },

    /// Start WebDAV server for remote access
    #[cfg(feature = "web")]
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Directory to serve
        #[arg(short, long, default_value = ".")]
        dir: String,
    },
}

pub fn run() -> i32 {
    crate::log::init();
    crate::log::debug("CLI", "Starting cf command");

    let cli = Cli::parse();

    match cli.command {
        Commands::CreateRoom { name } => commands::create_room(name.as_deref()),
        Commands::ListRooms => commands::list_rooms(),
        Commands::Register { chatfile, name } => commands::register(&chatfile, name.as_deref()),
        Commands::Join => commands::join(),
        Commands::Leave => commands::leave(),
        Commands::Send { message } => commands::send(&message),
        Commands::AdminSend { message } => commands::admin_send(&message),
        Commands::Await => commands::await_message(),
        Commands::SendAwait { message } => commands::send_await(&message),
        Commands::Read { n } => commands::read(n),
        Commands::Status => commands::status(),
        Commands::Clear {
            force,
            sessions_only,
        } => commands::clear(force, sessions_only),
        #[cfg(feature = "web")]
        Commands::Serve { port, dir } => commands::serve(port, &dir),
    }
}
