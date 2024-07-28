use config::Config;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use tokio::time::{sleep, Duration};

mod config;

const API_URL: &str = "https://discord.com/api/v9/channels";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting config.");
    let config = Config::new()?;

    println!("Generating headers..");
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "accept-encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("authorization", HeaderValue::from_str(&config.token)?);
    headers.insert("content-length", HeaderValue::from_static("0"));
    headers.insert("origin", HeaderValue::from_static("https://discord.com"));

    println!("Started loop.");
    loop {
        println!("Sent typing request.");
        client
            .post(format!(
                "{url}/{channel_id}/typing",
                url = API_URL,
                channel_id = config.channel_id
            ))
            .headers(headers.clone())
            .send()
            .await?;
        sleep(Duration::from_secs(config.interval_seconds)).await;
    }
}
