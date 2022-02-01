use clap::Parser;
use std::io::{Error, ErrorKind};
use reqwest::Url;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[clap(name = "wget")]
struct Cli {
    #[clap(short, long)]
    quiet: bool,

    url: String,
}

async fn fetch_web(url: Url) -> Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

async fn fetch(url: &str) -> Result<String> {
    let url = url.parse::<Url>()?;
    match url.scheme() {
        "http" | "https" => fetch_web(url).await,
        s => Err(Box::new(Error::new(ErrorKind::Unsupported, format!("unsupported scheme: {}", s))))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match fetch(&args.url).await {
        Ok(r) => println!("{r}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
    
    Ok(())
}
