use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 2021)]
    year: i32,

    #[arg(short, long, default_value_t = 1)]
    day: i32,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Fetch,
    Submit,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Fetch) => {
            todo!();
        }
        Some(Commands::Submit) => {
            todo!();
        }
        None => {
            println!("none");
        }
    }
    println!("all");
    println!("Value for year: {}", cli.year);
    println!("Value for day: {}", cli.day);
}
