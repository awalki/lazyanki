use clap::{Parser, Subcommand};
use lazyanki::commands::{init_command, list_command, new_command};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {
        #[arg(long, short)]
        native_language: String,
        #[arg(long, short)]
        target_language: String,
    },
    New {
        #[arg(long, short)]
        word: String,
        #[arg(long, short)]
        deck: String,
    },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New { word, deck } => new_command(word, deck).await?,
        Commands::Init {
            native_language,
            target_language,
        } => init_command(native_language, target_language).await?,
        Commands::List => list_command().await?,
    }

    Ok(())
}
