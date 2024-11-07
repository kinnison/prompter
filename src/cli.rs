//! The CLI

use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Copy)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand, Clone, Copy)]
pub enum Command {
    Init,
}
