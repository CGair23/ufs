//!
//! The configuration is first loaded from `config/default.toml` and
//! then overwritten by the values in `config/local.toml`.
use std::path::Path;
use anyhow::Result;
use std::fs;

use serde::Deserialize;

/// Runtime configurations
#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub server_config: ServerConfig,
    // pub tls_config: TlsConfig,
}

/// Configurations for the http server.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub ip: String,
    pub port: u32,
}

/// Configurations for tls.
#[derive(Debug, Deserialize)]
pub struct TlsConfig {
    pub enable: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
}

impl ServerConfig {
    pub fn from_toml<T: AsRef<Path>>(path: T) -> Result<Self> {
        let contents = fs::read_to_string(path.as_ref())
            .context("Something went wrong when reading the runtime config file")?;
        let config: RuntimeConfig = 
            toml::from_str(&contents).context("Cannot parse the runtime config file")?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeConfig;
    #[test]
    fn test_parse_toml() {
        let config: RuntimeConfig = toml::from_str(r#"
            title = 'TOML Example'

            [server_config]
            host = "xxxxxxxxxxxxxxxxx"
            ip = "127.0.0.1"
            port = 8080

            [tls_config]
        "#).unwrap();
        assert_eq!(config.server_config.ip, "127.0.0.1");
        assert_eq!(config.server_config.host, "xxxxxxxxxxxxxxxxx");
        assert_eq!(config.server_config.port, 8080);
    }
}