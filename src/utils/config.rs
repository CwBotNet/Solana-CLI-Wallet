//! Configuration managemnet

use crate::utils::{Result, WalletError};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{default, env::home_dir, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    /// Default network (devnet, testnet, mainnet)
    pub default_network: String,
    /// Directory where keypairs are stored
    pub keypair_dir: PathBuf,
    /// Default RPC endpoint for each network
    pub rpc_endpoints: NetworkEndpoints,
    /// Display preferences
    pub display_settings: DisplaySettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkEndpoints {
    pub devnet: String,
    pub testnet: String,
    pub mainnet: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplaySettings {
    /// Numberr pf decimal places for sol amounts
    pub sol_precision: u8,
    /// Show verbose transaction details
    pub verbose_transactions: bool,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            default_network: "devnet".to_string(),
            keypair_dir: Self::default_keypair_dir(),
            rpc_endpoints: NetworkEndpoints {
                devnet: "https://api.devnet.solana.com".to_string(),
                testnet: "https://api.testnet.solana.com".to_string(),
                mainnet: "https://api.mainnet-beta.solana.com".to_string(),
            },
            display_settings: DisplaySettings {
                sol_precision: 9,
                verbose_transactions: false,
            },
        }
    }
}

impl WalletConfig {
    /// Get the default configuration directory
    pub fn config_dir() -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| {
            WalletError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find home directory",
            ))
        })?;

        Ok(home.join(".solana-wallet"))
    }

    /// Get the default keypair storage directory
    fn default_keypair_dir() -> PathBuf {
        Self::config_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("keypairs")
    }

    /// Load configuration form file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_dir()?.join("config.json");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: WalletConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;

        // Create directory if it doesn't exist (secure permissions)
        std::fs::create_dir_all(&config_dir)?;

        // Create keypair directory too
        std::fs::create_dir_all(&self.keypair_dir)?;

        let config_path = config_dir.join("config.json");
        let content = serde_json::to_string_pretty(self)?;

        std::fs::write(&config_path, contents)?;

        Ok(())
    }

    /// Get RPC endpoint for specified network
    pub fn get_rpc_endpoints(&self, network: &str) -> &str {
        match network {
            "devnet" => &self.rpc_endpoints.devnet,
            "testnet" => &self.rpc_endpoints.testnet,
            "mainnet" => &self.rpc_endpoints.mainnet,
            _ => &self.rpc_endpoints.devnet, // default fallback
        }
    }
}
