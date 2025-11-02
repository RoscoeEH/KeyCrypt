use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "seclock", version, about = "Device locking exchange")]
/// The top-level command to execute.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Server,
    Client,
}
