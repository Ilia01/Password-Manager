use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
        #[arg(short, long, default_missing_value = "true")]
        clipboard: bool,
        #[arg(short, long, default_missing_value = "true")]
        generate: bool,
        #[arg(short, long, default_missing_value = "true")]
        write: bool,
    },
}
