# Quick Start Script for C2 and Reverse Shell Servers

Write-Host "`n╔═══════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   C2 & Reverse Shell Server Quick Start          ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

# Check if merino is installed
if (!(Get-Command merino -ErrorAction SilentlyContinue)) {
    Write-Host "⚠  Merino not found. Installing..." -ForegroundColor Yellow
    cargo install merino
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ Failed to install merino" -ForegroundColor Red
        exit 1
    }
    Write-Host "✓ Merino installed successfully" -ForegroundColor Green
}

# Create necessary directories
Write-Host "`n📁 Creating directories..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path "configs" | Out-Null
New-Item -ItemType Directory -Force -Path "certs" | Out-Null
New-Item -ItemType Directory -Force -Path "logs" | Out-Null

# Generate configurations if they don't exist
if (!(Test-Path "configs\c2-server.toml")) {
    Write-Host "📝 Generating C2 server configuration..." -ForegroundColor Cyan
    cargo run --bin c2-server -- --generate-config
}

if (!(Test-Path "configs\reverse-shell.toml")) {
    Write-Host "📝 Generating reverse shell configuration..." -ForegroundColor Cyan
    cargo run --bin shell-server -- --generate-config
}

if (!(Test-Path "configs\merino-users.csv")) {
    Write-Host "📝 Creating default SOCKS5 users..." -ForegroundColor Cyan
    @"
username,password
c2agent,Ch@ng3M3!Str0ng
operator1,S3cur3P@ssw0rd!
admin,Adm1n!P@ssw0rd
"@ | Out-File -FilePath "configs\merino-users.csv" -Encoding ASCII
}

Write-Host "`n✓ Setup complete!" -ForegroundColor Green
Write-Host "`n⚠  IMPORTANT: Edit configs and change default passwords!" -ForegroundColor Yellow
Write-Host "   - configs\c2-server.toml" -ForegroundColor Yellow
Write-Host "   - configs\reverse-shell.toml" -ForegroundColor Yellow  
Write-Host "   - configs\merino-users.csv" -ForegroundColor Yellow

Write-Host "`n" -NoNewline
$choice = Read-Host "Start servers now? (y/n)"

if ($choice -eq 'y' -or $choice -eq 'Y') {
    Write-Host "`n🚀 Starting servers..." -ForegroundColor Cyan
    
    # Build first
    Write-Host "`n🔨 Building..." -ForegroundColor Cyan
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ Build failed" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "`n✓ Build successful" -ForegroundColor Green
    
    # Start C2 server in new window
    Write-Host "🚀 Launching C2 server..." -ForegroundColor Cyan
    Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd '$PWD'; .\target\release\c2-server.exe"
    
    Start-Sleep -Seconds 2
    
    # Start Shell server in new window
    Write-Host "🚀 Launching Reverse Shell server..." -ForegroundColor Cyan
    Start-Process pwsh -ArgumentList "-NoExit", "-Command", "cd '$PWD'; .\target\release\shell-server.exe"
    
    Write-Host "`n✓ Servers started in separate windows" -ForegroundColor Green
    Write-Host "`n📊 Server Info:" -ForegroundColor Cyan
    Write-Host "   C2 Server: https://0.0.0.0:8443" -ForegroundColor White
    Write-Host "   Shell Server: tls://0.0.0.0:4444" -ForegroundColor White
    Write-Host "   SOCKS5 Proxy: socks5://127.0.0.1:1080" -ForegroundColor White
    Write-Host "   SOCKS5 Proxy (Shell): socks5://127.0.0.1:1081" -ForegroundColor White
} else {
    Write-Host "`nTo start servers manually:" -ForegroundColor Cyan
    Write-Host "   cargo run --bin c2-server" -ForegroundColor White
    Write-Host "   cargo run --bin shell-server" -ForegroundColor White
}

Write-Host ""
