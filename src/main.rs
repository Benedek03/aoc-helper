use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug)]
struct Config {
    session_id: String,
    root_dir: PathBuf,
}

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

async fn d(){
    let target = "https://raw.githubusercontent.com/Benedek03/aoc/main/README.md";
    let response = reqwest::get(target).await;
    match response {
        Ok(r) => println!()
    }
    dbg!(response);
}

fn main() {
    let proj_dirs = ProjectDirs::from("dev", "Benedek03", "aoc-helper").unwrap();
    let config_dir = proj_dirs.config_dir();
    let config_file = fs::read_to_string(config_dir.join("config.toml"));
    let config: Config = match config_file {
        Ok(file) => {
            let a: Result<Config, toml::de::Error> = toml::from_str(&file);
            match a {
                Ok(c) => {
                    if ! c.root_dir.is_dir() {
                        println!("config issue:");
                        println!("root_dir must be a dir");
                        std::process::exit(1);
                    }
                    c
                },
                Err(_) => {
                    println!("config issue");
                    std::process::exit(1);
                }
            }
        }
        Err(_) => {
            println!(
r#"——————————No config?——————————
⣞⢽⢪⢣⢣⢣⢫⡺⡵⣝⡮⣗⢷⢽⢽⢽⣮⡷⡽⣜⣜⢮⢺⣜⢷⢽⢝⡽⣝
⠸⡸⠜⠕⠕⠁⢁⢇⢏⢽⢺⣪⡳⡝⣎⣏⢯⢞⡿⣟⣷⣳⢯⡷⣽⢽⢯⣳⣫⠇
⠀⠀⢀⢀⢄⢬⢪⡪⡎⣆⡈⠚⠜⠕⠇⠗⠝⢕⢯⢫⣞⣯⣿⣻⡽⣏⢗⣗⠏⠀
⠀⠪⡪⡪⣪⢪⢺⢸⢢⢓⢆⢤⢀⠀⠀⠀⠀⠈⢊⢞⡾⣿⡯⣏⢮⠷⠁⠀⠀
⠀⠀⠀⠈⠊⠆⡃⠕⢕⢇⢇⢇⢇⢇⢏⢎⢎⢆⢄⠀⢑⣽⣿⢝⠲⠉⠀⠀⠀⠀
⠀⠀⠀⠀⠀⡿⠂⠠⠀⡇⢇⠕⢈⣀⠀⠁⠡⠣⡣⡫⣂⣿⠯⢪⠰⠂⠀⠀⠀⠀
⠀⠀⠀⠀⡦⡙⡂⢀⢤⢣⠣⡈⣾⡃⠠⠄⠀⡄⢱⣌⣶⢏⢊⠂⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢝⡲⣜⡮⡏⢎⢌⢂⠙⠢⠐⢀⢘⢵⣽⣿⡿⠁⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠨⣺⡺⡕⡕⡱⡑⡆⡕⡅⡕⡜⡼⢽⡻⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣼⣳⣫⣾⣵⣗⡵⡱⡡⢣⢑⢕⢜⢕⡝⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣴⣿⣾⣿⣿⣿⡿⡽⡑⢌⠪⡢⡣⣣⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡟⡾⣿⢿⢿⢵⣽⣾⣼⣘⢸⢸⣞⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠁⠇⠡⠩⡫⢿⣝⡻⡮⣒⢽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
——————————————————————————————"#
            );
            std::process::exit(1);
        }
    };

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
        None =>  {
            println!("none");
            d();
        }
    }

}