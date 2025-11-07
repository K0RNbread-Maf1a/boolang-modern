# C2 & Reverse Shell - Quick Start Guide

## 📦 What Was Created

### Core Servers
- **C2 Server** - Command & Control server with agent management
- **Reverse Shell Server** - TLS-encrypted reverse shell listener  
- **Merino Integration** - SOCKS5 proxy management using your installed merino binary

### Features
✅ **TLS Encryption** - Self-signed certificates auto-generated  
✅ **SOCKS5 Proxy** - Merino proxy with CSV-based authentication  
✅ **Configuration** - TOML configs for easy customization  
✅ **Logging** - Comprehensive logging to files and console  
✅ **Async I/O** - Built on Tokio for high performance  

## 🚀 Getting Started

### Option 1: Use the Quick Start Script

```powershell
cd C:\Users\redgh\boolang-modern\crates\c2-server
.\start-servers.ps1
```

This script will:
1. Check if merino is installed (install if needed)
2. Create necessary directories
3. Generate default configurations
4. Optionally build and start both servers

### Option 2: Manual Start

```powershell
cd C:\Users\redgh\boolang-modern\crates\c2-server

# Build
cargo build --release

# Start C2 server
.\target\release\c2-server.exe

# Start reverse shell server (in another terminal)
.\target\release\shell-server.exe
```

## 📁 Project Structure

```
crates/c2-server/
├── Cargo.toml              # Dependencies and build config
├── README.md               # Full documentation
├── QUICKSTART.md           # This file
├── start-servers.ps1       # Quick start script
│
├── configs/
│   ├── c2-server.toml      # C2 server configuration
│   ├── reverse-shell.toml  # Shell server configuration
│   └── merino-users.csv    # SOCKS5 authentication
│
├── src/
│   ├── lib.rs              # Library exports
│   ├── c2.rs               # C2 server implementation
│   ├── shell.rs            # Reverse shell implementation
│   ├── certbot.rs          # TLS certificate management
│   ├── config.rs           # Configuration structures
│   ├── socks5.rs           # Merino SOCKS5 manager
│   │
│   └── bin/
│       ├── c2-server.rs    # C2 server binary
│       └── shell-server.rs # Shell server binary
│
├── certs/                  # Auto-generated TLS certificates
└── logs/                   # Server logs
```

## ⚙️ Configuration

### Before First Run

**CRITICAL:** Change default passwords in `configs/merino-users.csv`:

```csv
username,password
c2agent,YOUR_STRONG_PASSWORD_HERE
operator1,ANOTHER_STRONG_PASSWORD
```

**IMPORTANT:** Update auth tokens in `configs/c2-server.toml`:

```toml
[security]
auth_token = "YOUR-SECURE-TOKEN-HERE"
encryption_key = "YOUR-BASE64-KEY-HERE"
```

Generate secure tokens:
```powershell
# Generate random token
[Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Maximum 256 }))
```

## 🔌 Connecting

### C2 Server
- **URL:** `https://localhost:8443`
- **Protocol:** TLS over TCP
- **Format:** JSON messages
- **SOCKS5:** `socks5://127.0.0.1:1080`

### Reverse Shell Server
- **URL:** `tls://localhost:4444`
- **Protocol:** TLS over TCP
- **Shell:** PowerShell (Windows) or Bash (Linux)
- **SOCKS5:** `socks5://127.0.0.1:1081`

## 📝 Common Commands

### Build
```powershell
cargo build              # Debug build
cargo build --release    # Optimized release build
```

### Run
```powershell
cargo run --bin c2-server                    # Run C2 server
cargo run --bin shell-server                 # Run shell server
cargo run --bin c2-server -- --help          # Show help
cargo run --bin c2-server -- --generate-config  # Generate config
```

### Test Merino SOCKS5
```powershell
# Check if merino is running
Get-Process merino

# Test SOCKS5 connection
curl --socks5 127.0.0.1:1080 --socks5-user c2agent:Ch@ng3M3!Str0ng https://example.com
```

### View Logs
```powershell
Get-Content logs\c2.log -Tail 50 -Wait     # Tail C2 logs
Get-Content logs\shell.log -Tail 50 -Wait  # Tail shell logs
```

## 🔧 Troubleshooting

### Merino not found
```powershell
cargo install merino
```

### Port already in use
Edit configs and change ports:
```toml
[server]
port = 9443  # Change from 8443
```

### Certificate errors
Delete certs and restart:
```powershell
Remove-Item -Recurse certs\
cargo run --bin c2-server
```

### Can't bind to port
Run as Administrator or use ports > 1024

## 🛡️ Security Warnings

⚠️ **FOR AUTHORIZED TESTING ONLY**

- Only use on systems you own or have permission to test
- Change ALL default credentials before deployment
- Use TLS in production environments
- Monitor logs for unauthorized access
- Comply with all applicable laws

## 📚 Next Steps

1. **Read the full README.md** for detailed documentation
2. **Customize configurations** in the `configs/` directory
3. **Build agent/implant code** to connect to servers
4. **Implement additional C2 features** (file transfer, screenshots, etc.)
5. **Add encryption** for C2 messages beyond TLS
6. **Create web UI** for C2 management

## 🔗 Related Documentation

- **Full README:** `README.md`
- **Merino SOCKS5:** https://github.com/ajmwagar/merino
- **Tokio Async:** https://tokio.rs
- **Rustls TLS:** https://github.com/rustls/rustls

## 📞 Support

For issues or questions:
1. Check the README.md troubleshooting section
2. Review logs in `logs/` directory
3. Verify merino is installed and accessible
4. Check firewall rules allow the ports

---

**Status:** ✅ Ready to deploy
**Version:** 0.1.0
**License:** MIT OR Apache-2.0
