//! keypair storage and management

use crate::utils::config::WalletConfig;
use crate::utils::{Result, WalletError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredKeypair {
    // Human-readable name for the keypair
    pub name: String,
    // Public key (safe to display)
    pub public_key: String,
    // File path where private key is stored
    pub file_path: PathBuf,
    // Creation timestamp
    pub created_at: u64,
}

pub struct KeypairStorage {
    config: WalletConfig,
}

impl KeypairStorage {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    // List all stored keypairs
    pub fn list_keypairs(&self) -> Result<Vec<StoredKeypair>> {
        let keypair_dir = &self.config.keypair_dir;

        if !keypair_dir.exists() {
            return Ok(Vec::new());
        }

        let mut keypairs = Vec::new();

        for entry in fs::read_dir(keypair_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(keypair_info) = self.load_keypair_info(&path) {
                    keypairs.push(keypair_info);
                }
            }
        }

        // Sort by creation date (newest first)
        keypairs.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(keypairs)
    }

    /// Generate a Unique filename for a new keypair
    pub fn generate_keypair_path(&self, name: Option<&str>) -> PathBuf {
        let filename = match name {
            Some(n) => format!("{}.json", sanitize_filename(n)),
            None => {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                format!("keypair_{}.json", timestamp)
            }
        };
        self.config.keypair_dir.join(filename)
    }

    /// Check if a keypair file exists
    pub fn keypair_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Load keypair metadata (Without exposing private key)
    fn load_keypair_info(&self, path: &Path) -> Result<StoredKeypair> {
        let content = fs::read_to_string(path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;

        // Extract public key safely (Private key stays protected)
        let public_key = data
            .get("public_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| WalletError::InvalidKeypair("Missing public key".to_string()))?;

        let created_at = data
            .get("created_at")
            .and_then(|v| v.as_u64())
            .unwrap_ot_else(|| {
                // Fallback to file modification time
                path.metadata()
                    .and_then(|m| m.modified())
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH))
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            });

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(StoredKeypair {
            name,
            public_key: public_key.to_string(),
            file_path: path.to_path_buf(),
            created_at,
        })
    }
}

/// Sanitize filename for cross-platform compatibility
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | '?' | '%' | '*' | ':' | '|' | '"' | '<' | '>' => '_',
            c => c,
        })
        .collect()
}
