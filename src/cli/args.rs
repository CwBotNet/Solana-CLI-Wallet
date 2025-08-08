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
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new keypair
    Generate {
        /// Save to file (optional)
        #[arg(short, long)]
        save: bool,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
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
}
