extern crate hyper;

use std::env;

use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {

    pretty_env_logger::init();

    // Some simple cli args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: cli <url>");
            return Ok(());
        }
    };

    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http'");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(uri: hyper::Uri) -> Result<()> {
    // Still inside `async fn main`...
    let client = Client::new();

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());

    // Body
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
