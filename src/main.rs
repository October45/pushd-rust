use reqwest::blocking::Client;
use std::io::stdin;
use std::process::exit;

mod consts;
use consts::{PUSHOVER_API_TOKEN, PUSHOVER_TITLE, PUSHOVER_USER};

fn main() {
    // Read message from stdin
    let lines = stdin().lines();
    for message in lines {
        let message = match message {
            Ok(message) => message,
            Err(e) => {
                eprintln!("Failed to read message: {}", e);
                exit(1);
            }
        };
        if message.trim().is_empty() {
            break;
        }
        // Send message to Pushover
        let client = Client::new();
        let response = client
            .post("https://api.pushover.net/1/messages.json")
            .form(&[
                ("token", PUSHOVER_API_TOKEN),
                ("user", PUSHOVER_USER),
                ("title", PUSHOVER_TITLE),
                ("message", message.trim()),
            ])
            .send();
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Message sent successfully");
                } else {
                    eprintln!("Failed to send message: {}", response.text().unwrap());
                    exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to send message: {}", e);
                exit(1);
            }
        }
    }
}
