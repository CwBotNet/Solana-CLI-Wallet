use anyhow::{Ok, Result};
use clap::parser;
use Solana_CLI_Wallet::{cli::Cli, wallet};

#[tokio::main]
async fn main()-> Result<()>{
    let cli =Cli::parser();

    Ok(())
}