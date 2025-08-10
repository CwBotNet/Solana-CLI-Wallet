use Solana_CLI_Wallet::cli::{self, commands::handle_command};
use Solana_CLI_Wallet::utils::Result;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    // Handle the command using modular structure
    handle_command(cli.command, cli.verbose, &cli.network).await?;
    Ok(())
}
