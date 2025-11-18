use crate::constants::*;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "seclock", version, about = "Device locking exchange")]
/// The top-level command to execute.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Server(ServerArgs),
    Client(ClientArgs),
    KeyGen(KeyGenArgs),
}

#[derive(Args, Clone)]
pub struct ServerArgs {
    #[arg(short = 'i', long = "ip", default_value_t = String::from(DEFAULT_IP_ADDRESS))]
    pub ip_addr: String,

    #[arg(short = 'k', long = "key", default_value_t = String::from(DEFAULT_PRIVATE_KEY_FILE))]
    pub key_path: String,
}

#[derive(Args, Clone)]
pub struct ClientArgs {
    #[arg(short = 'i', long = "ip", default_value_t = String::from(DEFAULT_IP_ADDRESS))]
    pub ip_addr: String,

    #[arg(short = 'k', long = "key", default_value_t = String::from(DEFAULT_PUBLIC_KEY_FILE))]
    pub key_path: String,
}

#[derive(Args, Clone)]
pub struct KeyGenArgs {
    #[arg(short = 'd', long = "decap-key", default_value_t = String::from(DEFAULT_PRIVATE_KEY_FILE))]
    pub decap_key_path: String,

    #[arg(short = 'e', long = "encap-key", default_value_t = String::from(DEFAULT_PUBLIC_KEY_FILE))]
    pub encap_key_path: String,
}
