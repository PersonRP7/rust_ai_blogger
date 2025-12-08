use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::time::Duration;
use dotenv::dotenv;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
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

    // ---- 1. Get file path from command line ----
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // ---- 2. Read file content ----
    let file_content = fs::read_to_string(filename)?;

    // Optional: log length so you see how big the prompt is
    println!("Read file `{}` ({} bytes)", filename, file_content.len());

    // ---- 3. Build the prompt ----
    let prompt = format!(
        "Format this content into HTML:\n\n{}",
        file_content
    );

    let url = "https://openrouter.ai/api/v1/chat/completions";

    // ---- 4. Client with a longer timeout ----
    let client = Client::builder()
        .timeout(Duration::from_secs(300)) // 5 minutes, adjust as needed
        .build()?;

    let request = ChatRequest {
        model: "tngtech/deepseek-r1t2-chimera:free".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()?;

    let text = res.text()?; // this was timing out before
    println!("Raw response:\n{}\n", text);

    let parsed: Result<ChatResponse, _> = serde_json::from_str(&text);
    match parsed {
        Ok(response) => {
            println!("{}", response.choices[0].message.content);
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
        }
    }

    Ok(())
}