use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2Config {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub socks5: Socks5Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub max_clients: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub auto_generate: bool,
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub auth_token: String,
    pub encryption_key: String,
    pub allowed_ips: Vec<String>,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub max_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<PathBuf>,
    pub console: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socks5Config {
    pub enabled: bool,
    pub ip: String,
    pub port: u16,
    pub users_csv: PathBuf,
    pub no_auth: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseShellConfig {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub shell: ShellConfig,
    pub logging: LoggingConfig,
    pub socks5: Socks5Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub command: String,
    pub args: Vec<String>,
    pub environment: Vec<ShellEnv>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellEnv {
    pub key: String,
    pub value: String,
}

impl Default for C2Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 8443,
                max_clients: 100,
                timeout_seconds: 300,
            },
            tls: TlsConfig {
                enabled: true,
                cert_path: PathBuf::from("certs/server.crt"),
                key_path: PathBuf::from("certs/server.key"),
                auto_generate: true,
                domain: Some("c2.local".to_string()),
            },
            security: SecurityConfig {
                auth_token: uuid::Uuid::new_v4().to_string(),
                encryption_key: base64::encode(rand::random::<[u8; 32]>()),
                allowed_ips: vec!["0.0.0.0/0".to_string()],
                rate_limit: RateLimitConfig {
                    enabled: true,
                    max_requests_per_minute: 60,
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: Some(PathBuf::from("logs/c2.log")),
                console: true,
            },
            socks5: Socks5Config {
                enabled: true,
                ip: "127.0.0.1".to_string(),
                port: 1080,
                users_csv: PathBuf::from("configs/merino-users.csv"),
                no_auth: false,
            },
        }
    }
}

impl Default for ReverseShellConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 4444,
                max_clients: 50,
                timeout_seconds: 600,
            },
            tls: TlsConfig {
                enabled: true,
                cert_path: PathBuf::from("certs/shell.crt"),
                key_path: PathBuf::from("certs/shell.key"),
                auto_generate: true,
                domain: Some("shell.local".to_string()),
            },
            shell: ShellConfig {
                #[cfg(windows)]
                command: "powershell.exe".to_string(),
                #[cfg(not(windows))]
                command: "/bin/bash".to_string(),
                args: vec![],
                environment: vec![],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: Some(PathBuf::from("logs/shell.log")),
                console: true,
            },
            socks5: Socks5Config {
                enabled: true,
                ip: "127.0.0.1".to_string(),
                port: 1081,
                users_csv: PathBuf::from("configs/merino-users.csv"),
                no_auth: false,
            },
        }
    }
}

impl C2Config {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl ReverseShellConfig {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
