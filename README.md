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
- `cf create-room [name]` - Create a room (`name.Chatfile` or `Chatfile`)
- `cf list-rooms` - List available rooms in current directory
- `cf register <chatfile> [--name NAME]` - Register with a chatfile
- `cf join` - Join the room (announces entry)
- `cf leave` - Leave the room (announces exit)

### Messaging
- `cf send "message"` - Send a message
- `cf await` - Wait for the next message
- `cf send-await "msg"` - Send and wait for reply
- `cf read [n]` - Show last n messages (default 20)

### Admin
- `cf admin-send "message"` - Send as admin (requires `.cf_admin` file)

### Utilities
- `cf status` - Show current session info
- `cf clear [--force] [--sessions-only]` - Remove chatfiles and sessions

## Example Usage

```bash
# Create a room
cf create-room dev

# Register with custom name and join
cf register dev.Chatfile --name MyAgent
cf join

# Send messages
cf send "Hello!"
cf await

# Leave when done
cf leave
```

## XDG Conformity

Sessions are stored in XDG-compliant locations:
- Sessions: `~/.local/share/chatfiles/sessions/<hash>.session`
- Config: `~/.config/chatfiles/`

Legacy `.cf_session` in CWD or home directory is still supported.

## Environment Variables

- `CF_SESSION` - Override session file path (useful for running multiple agents)

## Features

- `hyprlog` - Enable hyprlog integration for colored logging (optional)
- `web` - Enable WebDAV server for remote access (optional)

## License

MIT
