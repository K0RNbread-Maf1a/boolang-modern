use anyhow::{Context, Result};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, SanType};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tracing::{info, warn};

pub struct CertBot {
    cert_dir: PathBuf,
}

#[derive(Debug)]
pub struct CertificateInfo {
    pub domain: String,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl CertBot {
    pub fn new<P: AsRef<Path>>(cert_dir: P) -> Result<Self> {
        let cert_dir = cert_dir.as_ref().to_path_buf();
        fs::create_dir_all(&cert_dir)?;
        Ok(Self { cert_dir })
    }

    /// Generate a self-signed certificate
    pub fn generate_self_signed(
        &self,
        domain: &str,
        validity_days: u32,
    ) -> Result<CertificateInfo> {
        info!("Generating self-signed certificate for domain: {}", domain);

        let mut params = CertificateParams::default();
        
        // Set subject alternative names
        params.subject_alt_names = vec![
            SanType::DnsName(domain.to_string()),
            SanType::DnsName(format!("*.{}", domain)),
            SanType::IpAddress(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
        ];

        // Set distinguished name
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, domain);
        dn.push(DnType::OrganizationName, "C2 Server");
        dn.push(DnType::CountryName, "US");
        params.distinguished_name = dn;

        // Set validity period
        let not_before = SystemTime::now();
        let not_after = not_before + std::time::Duration::from_secs(validity_days as u64 * 86400);
        params.not_before = not_before.into();
        params.not_after = not_after.into();

        // Generate certificate
        let cert = Certificate::from_params(params)
            .context("Failed to generate certificate")?;

        // Get PEM strings
        let cert_pem = cert.serialize_pem()?;
        let key_pem = cert.serialize_private_key_pem();

        // Save to files
        let cert_path = self.cert_dir.join(format!("{}.crt", domain.replace("*", "wildcard")));
        let key_path = self.cert_dir.join(format!("{}.key", domain.replace("*", "wildcard")));

        fs::write(&cert_path, cert_pem.as_bytes())?;
        fs::write(&key_path, key_pem.as_bytes())?;

        info!("Certificate saved to: {:?}", cert_path);
        info!("Private key saved to: {:?}", key_path);

        Ok(CertificateInfo {
            domain: domain.to_string(),
            cert_path,
            key_path,
        })
    }

    /// Load certificate and key for TLS
    pub fn load_tls_config(
        cert_path: &Path,
        key_path: &Path,
    ) -> Result<rustls::ServerConfig> {
        let cert_file = File::open(cert_path)
            .context("Failed to open certificate file")?;
        let mut cert_reader = BufReader::new(cert_file);

        let key_file = File::open(key_path)
            .context("Failed to open key file")?;
        let mut key_reader = BufReader::new(key_file);

        let certs = certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse certificates")?;

        let mut keys = pkcs8_private_keys(&mut key_reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse private keys")?;

        if keys.is_empty() {
            anyhow::bail!("No private keys found");
        }

        let key = keys.remove(0);

        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key.into())
            .context("Failed to create TLS config")?;

        Ok(config)
    }

    /// Auto-generate or load existing certificate
    pub fn ensure_certificate(
        &self,
        domain: &str,
        cert_path: &Path,
        key_path: &Path,
    ) -> Result<CertificateInfo> {
        if cert_path.exists() && key_path.exists() {
            info!("Found existing certificate for {}", domain);
            return Ok(CertificateInfo {
                domain: domain.to_string(),
                cert_path: cert_path.to_path_buf(),
                key_path: key_path.to_path_buf(),
            });
        }

        info!("No existing certificate found for {}, generating new one", domain);
        self.generate_self_signed(domain, 365)
    }
}
