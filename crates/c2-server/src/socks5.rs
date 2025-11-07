use crate::config::Socks5Config;
use anyhow::{Context, Result};
use std::process::{Child, Command, Stdio};
use tracing::{info, warn, error};

/// Manages merino SOCKS5 proxy instances
pub struct MerinoManager {
    config: Socks5Config,
    process: Option<Child>,
}

impl MerinoManager {
    pub fn new(config: Socks5Config) -> Self {
        Self {
            config,
            process: None,
        }
    }

    /// Start the merino SOCKS5 proxy server
    pub fn start(&mut self) -> Result<()> {
        if !self.config.enabled {
            info!("SOCKS5 proxy disabled in configuration");
            return Ok(());
        }

        info!(
            "Starting merino SOCKS5 proxy on {}:{}",
            self.config.ip, self.config.port
        );

        let mut cmd = Command::new("merino");
        
        cmd.arg("--ip")
            .arg(&self.config.ip)
            .arg("--port")
            .arg(self.config.port.to_string());

        if self.config.no_auth {
            cmd.arg("--no-auth");
        } else if self.config.users_csv.exists() {
            cmd.arg("--users")
                .arg(&self.config.users_csv);
            info!("Using authentication with users from: {:?}", self.config.users_csv);
        } else {
            warn!("Users CSV file not found: {:?}", self.config.users_csv);
            warn!("Starting with no authentication");
            cmd.arg("--no-auth");
        }

        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd.spawn()
            .context("Failed to start merino process")?;

        info!("Merino SOCKS5 proxy started (PID: {})", child.id());
        self.process = Some(child);

        Ok(())
    }

    /// Stop the merino SOCKS5 proxy server
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            info!("Stopping merino SOCKS5 proxy (PID: {})", process.id());
            
            process.kill()
                .context("Failed to kill merino process")?;
            
            process.wait()
                .context("Failed to wait for merino process")?;
            
            info!("Merino SOCKS5 proxy stopped");
        }
        Ok(())
    }

    /// Check if the proxy is running
    pub fn is_running(&mut self) -> bool {
        if let Some(process) = &mut self.process {
            match process.try_wait() {
                Ok(Some(_)) => {
                    error!("Merino process unexpectedly terminated");
                    self.process = None;
                    false
                }
                Ok(None) => true,
                Err(e) => {
                    error!("Failed to check merino process status: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Get SOCKS5 proxy URL
    pub fn proxy_url(&self) -> String {
        format!("socks5://{}:{}", self.config.ip, self.config.port)
    }
}

impl Drop for MerinoManager {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Error stopping merino on drop: {}", e);
        }
    }
}

/// Generate a CSV users file from username/password pairs
pub fn generate_users_csv(
    path: &std::path::Path,
    users: &[(String, String)],
) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::File::create(path)
        .context("Failed to create users CSV file")?;
    
    writeln!(file, "username,password")?;
    
    for (username, password) in users {
        writeln!(file, "{},{}", username, password)?;
    }
    
    info!("Generated users CSV at: {:?}", path);
    Ok(())
}

/// Parse users from CSV file
pub fn parse_users_csv(path: &std::path::Path) -> Result<Vec<(String, String)>> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read users CSV file")?;
    
    let mut users = Vec::new();
    
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // Skip header
        }
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            users.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    
    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::path::PathBuf;

    #[test]
    fn test_generate_and_parse_users_csv() {
        let temp_dir = tempdir().unwrap();
        let csv_path = temp_dir.path().join("users.csv");
        
        let users = vec![
            ("alice".to_string(), "password1".to_string()),
            ("bob".to_string(), "password2".to_string()),
        ];
        
        generate_users_csv(&csv_path, &users).unwrap();
        
        let parsed = parse_users_csv(&csv_path).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].0, "alice");
        assert_eq!(parsed[1].0, "bob");
    }
}
