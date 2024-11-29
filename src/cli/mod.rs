pub mod init;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

// Re-export init handler
pub use self::init::handle_init;

#[derive(Parser)]
#[command(name = "tbd")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional config file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new TBD project
    Init {
        /// Project name
        #[arg(short, long)]
        name: String,

        /// Project directory
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },

    /// Plan infrastructure changes
    Plan {
        /// Stack name
        #[arg(short, long)]
        stack: String,

        /// Workspace directory
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Apply infrastructure changes
    Apply {
        /// Stack name
        #[arg(short, long)]
        stack: String,

        /// Auto-approve changes
        #[arg(short, long)]
        auto_approve: bool,
    },

    /// Destroy infrastructure
    Destroy {
        /// Stack name
        #[arg(short, long)]
        stack: String,

        /// Auto-approve destruction
        #[arg(short, long)]
        auto_approve: bool,
    },

    /// Show current state
    Show {
        /// Stack name
        #[arg(short, long)]
        stack: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Provider management commands
    Provider {
        #[command(subcommand)]
        command: ProviderCommands,
    },
}

#[derive(Subcommand)]
enum ProviderCommands {
    /// List installed providers
    List,

    /// Install a provider
    Install {
        /// Provider name
        name: String,

        /// Provider version
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Remove a provider
    Remove {
        /// Provider name
        name: String,
    },
}

// Main CLI handler
pub async fn handle_cli() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging based on debug level
    match cli.debug {
        0 => tracing_subscriber::fmt().init(),
        1 => tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init(),
        2.. => tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init(),
    }

    match &cli.command {
        Commands::Init { name, dir } => init::handle_init(name, dir).await,
        Commands::Plan {
            stack: _stack,
            dir: _dir,
            format: _format,
        } => {
            todo!("Implement plan command")
        }
        Commands::Apply {
            stack: _stack,
            auto_approve: _auto_approve,
        } => {
            todo!("Implement apply command")
        }
        Commands::Destroy {
            stack: _stack,
            auto_approve: _auto_approve,
        } => {
            todo!("Implement destroy command")
        }
        Commands::Show {
            stack: _stack,
            format: _format,
        } => {
            todo!("Implement show command")
        }
        Commands::Provider { command } => match command {
            ProviderCommands::List => {
                todo!("Implement provider list command")
            }
            ProviderCommands::Install {
                name: _name,
                version: _version,
            } => {
                todo!("Implement provider install command")
            }
            ProviderCommands::Remove { name: _name } => {
                todo!("Implement provider remove command")
            }
        },
    }
}
