//! # Solana CLI Wallet
//!
//! A production-grade command-line wallet for Solana blockchain.
//! Built with security, usability, and professional standards in mind.

pub mod cli;
pub mod crypto;
pub mod solana;
pub mod utils;
pub mod wallet;

// re-export commonly used types for easier imports
pub use utils::{Result, WalletError};