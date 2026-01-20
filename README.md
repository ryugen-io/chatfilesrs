# chatfiles

Minimal text-file-based protocol for multi-agent coordination.

```
5 rules.

1. The file is Chatfile. Like dockerfile, prefix.Chatfile or simply Chatfile.
2. There should be one message in the chat file that explains how the chatfile works.
3. Syntax: `<name>: message\n`
4. One message, one line.
5. The only allowed operations are read and append.
```

---

## Installation

```bash
cargo install --git https://github.com/ryugen-io/chatfilesrs --features hyprlog
```

Or build from source:

```bash
cargo build --release --features hyprlog
cp target/release/cf ~/.local/bin/
```

## Commands

### Room Management
| Command | Aliases | Description |
|---------|---------|-------------|
| `cf create-room [name]` | `create`, `cr` | Create a room (`name.Chatfile` or `Chatfile`) |
| `cf list-rooms` | `list`, `ls` | List available rooms in current directory |
| `cf register <chatfile> [-n NAME]` | `reg`, `r` | Register with a chatfile |
| `cf join` | `j` | Join the room (announces entry) |
| `cf leave` | `l` | Leave the room (announces exit) |

### Messaging
| Command | Aliases | Description |
|---------|---------|-------------|
| `cf send "message"` | `s` | Send a message |
| `cf await` | `a`, `wait`, `w` | Wait for the next message |
| `cf send-await "msg"` | `sa` | Send and wait for reply |
| `cf read [n]` | `cat` | Show last n messages (default 20) |

### Admin
| Command | Aliases | Description |
|---------|---------|-------------|
| `cf admin-send "message"` | `as`, `admin` | Send as admin (requires `.cf_admin` file) |

### Utilities
| Command | Aliases | Description |
|---------|---------|-------------|
| `cf status` | `st` | Show current session info |
| `cf clear [-f] [-s]` | `cls`, `clean` | Remove chatfiles and sessions |

### Web (requires `--features web`)
| Command | Description |
|---------|-------------|
| `cf serve [-p PORT] [-d DIR]` | Start WebDAV server for remote access |

### Options

**register:**
- `-n, --name <NAME>` - Custom display name (default: random name like `swift-fox-1234`)

**clear:**
- `-f, --force` - Force deletion without confirmation
- `-s, --sessions-only` - Only delete session files, keep Chatfiles

**serve** (requires `--features web`):
- `-p, --port <PORT>` - Port to listen on (default: 8080)
- `-d, --dir <DIR>` - Directory to serve (default: current directory)

## Example Usage

```bash
# Create a room
cf cr dev

# Register with custom name and join
cf r dev.Chatfile -n MyAgent
cf j

# Send messages
cf s "Hello!"
cf w

# Read last 10 messages
cf cat 10

# Leave when done
cf l
```

## XDG Conformity

Sessions are stored in XDG-compliant locations:
- Sessions: `~/.local/share/chatfiles/sessions/<hash>.session`
- Config: `~/.config/chatfiles/`

Legacy `.cf_session` in CWD or home directory is still supported.

## Environment Variables

- `CF_SESSION` - Override session file path (useful for running multiple agents)

## Optional Features

Features are enabled at compile time via `--features`:

```bash
# Colored logging via hyprlog
cargo build --release --features hyprlog

# WebDAV server for remote access
cargo build --release --features web

# Both features
cargo build --release --features "hyprlog,web"
```

| Feature | Description |
|---------|-------------|
| `hyprlog` | Colored logging via [hyprlog](https://github.com/ryugen-io/hyprlog). Falls back to plain `eprintln` if disabled. |
| `web` | Enables `cf serve` command for WebDAV server access to chatfiles. |

## License

MIT
