mod config;
use config::*;
mod args;
use args::*;
use clap::Parser;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
    redirect::Policy,
    Client,
};
use std::io::Write;
use std::{
    fs::{self, File},
    path::PathBuf,
};

async fn download(client: Client, url: String, path: PathBuf) {
    let a = client.get(url).send().await.unwrap().text().await.unwrap();
    println!("{:?}", path);
    let mut output = File::create(path).unwrap();
    write!(output, "{}", a).unwrap();
}

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let cli = Cli::parse();
    doublecheck_day(&cli);

    match &cli.command {
        Some(Commands::Fetch) => {
            let cookie_header =
                HeaderValue::from_str(&format!("session={}", config.session)).unwrap();
            let content_type_header = HeaderValue::from_str("text/plain").unwrap();
            let mut headers = HeaderMap::new();
            headers.insert(COOKIE, cookie_header);
            headers.insert(CONTENT_TYPE, content_type_header);
            let client = Client::builder()
                .default_headers(headers)
                .redirect(Policy::none())
                .build()
                .unwrap();

            let url = format!(
                "https://adventofcode.com/{}/day/{}/input",
                cli.year, cli.day
            );

            let dir = config
                .root_dir
                .join(format!("{}/day{}/", cli.year, cli.day));
            if !dir.is_dir() {
                match fs::create_dir_all(&dir) {
                    Ok(()) => download(client, url, dir.join("input.txt")).await,
                    Err(e) => println!("{}", e),
                }
            }
        }
        Some(Commands::Submit) => {
            todo!();
        }
        Some(Commands::Open) => {
            let url = format!("https://adventofcode.com/{}/day/{}", cli.year, cli.day);
            match webbrowser::open(&url) {
                Ok(()) => println!("opened https://adventofcode.com/{}/day/{} in the default browser", cli.year, cli.day),
                Err(e) => println!("{}", e),
            }
        }
        None => {
            todo!();
        }
    }
}
