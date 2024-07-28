use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::{error::Error, fs};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub interval_seconds: u64,
    pub token: String,
    pub channel_id: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config: Config = match fs::read_to_string("Config.toml") {
            Ok(contents) => toml::from_str(&contents)?,
            Err(_) => {
                println!("Couldn't find `Config.toml`. Creating.");

                let config = Config::ask_questions();

                Config::create_config(&config)?;

                config
            }
        };

        Ok(config)
    }

    fn ask_questions() -> Config {
        // Ask for token
        println!("Please input your Discord token.");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut token = String::new();
        io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line.");
        let token = token.trim().to_string();

        // Ask for interval_second
        println!("Please input how many seconds you want to wait before sending type request again (Note: If you set this value too low, you might get rate-limited.).");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut interval_seconds = String::new();
        io::stdin()
            .read_line(&mut interval_seconds)
            .expect("Failed to read line.");
        let interval_seconds = interval_seconds
            .trim()
            .parse::<u64>()
            .expect("Failed to parse interval_seconds. Did you input a number?");

        // Ask channel_id
        println!("Please input the id of the channel you want to send requests to.");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut channel_id = String::new();
        io::stdin()
            .read_line(&mut channel_id)
            .expect("Failed to read line.");
        let channel_id = channel_id.trim().to_string();

        println!("Thank you.");

        Config {
            token,
            interval_seconds,
            channel_id,
        }
    }

    fn create_config(config: &Config) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(&config).unwrap();
        let mut file = fs::File::create("Config.toml")?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    }
}
