use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize, Debug)]
struct MessageResponse {
    role: String,
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY not set in environment variables");

    let client = Client::new();
    let url = "https://openrouter.ai/api/v1/chat/completions";

    let request = ChatRequest {
        model: "tngtech/deepseek-r1t2-chimera:free", // ✅ you can change this to any OpenRouter model
        messages: vec![Message {
            role: "user",
            content: "Hello World",
        }],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()?;

    let text = res.text()?;
    println!("Raw response:\n{}\n", text);

    // Try parsing into our struct
    let parsed: Result<ChatResponse, _> = serde_json::from_str(&text);
    match parsed {
        Ok(response) => {
            println!("Assistant: {}", response.choices[0].message.content);
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
        }
    }

    Ok(())
}