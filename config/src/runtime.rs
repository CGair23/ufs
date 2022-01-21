//!
//! The configuration is first loaded from `config/default.toml` and
//! then overwritten by the values in `config/local.toml`.
use std::path::Path;
use anyhow::{Result, Context};
use std::fs;

use serde::Deserialize;

/// Runtime configurations
#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub server_config: ServerConfig,
    // pub tls_config: TlsConfig,
    pub fs_root: String
}

/// Configurations for the http server.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub ip: String,
    pub port: u16,
}

/// Configurations for tls.
#[derive(Debug, Deserialize)]
pub struct TlsConfig {
    pub enable: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
}

impl RuntimeConfig {
    pub fn from_toml<T: AsRef<Path>>(path: T) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .context("Something went wrong when reading the runtime config file")?;
        let config: RuntimeConfig = 
            toml::from_str(&contents).context("Cannot parse the runtime config file")?;
        Ok(config)
    }

    pub fn server_host(&mut self, host: String) {
        self.server_config.server_host(host);
    }

    pub fn server_ip(&mut self, ip: String) {
        self.server_config.server_ip(ip);
    }

    pub fn server_port(&mut self, port: u16) {
        self.server_config.server_port(port);
    }
}

impl ServerConfig {
    pub fn server_host(&mut self, host: String) {
        self.host = host;
    }

    pub fn server_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn server_port(&mut self, port: u16) {
        self.port = port;
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeConfig;
    #[test]
    fn test_parse_toml() {
        let config: RuntimeConfig = toml::from_str(r#"
            title = 'TOML Example'

            fs_root = "SMLNODE/fs"

            [server_config]
            host = "xxxxxxxxxxxxxxxxx"
            ip = "127.0.0.1"
            port = 8080

            [tls_config]
            enabled = false
        "#).unwrap();
        assert_eq!(config.server_config.ip, "127.0.0.1");
        assert_eq!(config.server_config.host, "xxxxxxxxxxxxxxxxx");
        assert_eq!(config.server_config.port, 8080);
        assert_eq!(config.fs_root, "SMLNODE/fs");
    }
}