//! Command-line argument parsing

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wallet")]
#[command(about = "A professional Solana CLI wallet")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Network to use (devnet, testnet, mainnet)
    #[arg(short, long, default_value = "devnet")]
    pub network: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new keypair (like creating new credentials)
    Generate {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Force overwrite existing file
        #[arg(short, long)]
        force: bool,
    },

    /// Check account balance
    Balance {
        /// Account address to check
        address: String,

        /// Network to use (devnet, testnet, mainnet)
        #[arg(short, long, default_value = "devnet")]
        network: String,
    },

    /// Send SOL to another account
    Send {
        /// Recipient address
        to: String,

        /// Amount in SOL
        amount: f64,

        /// Private key file path
        #[arg(short, long)]
        keypair: String,
    },

    /// List all saved keypaiers (like your credential listing)
    List,
}
