use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 2022)]
    pub year: i32,

    #[arg(short, long, default_value_t = 1)]
    pub day: i32,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Fetch,
    Submit,
    Open,
}
