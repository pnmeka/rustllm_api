
use futures_util::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
struct Choice {
    delta: Delta,
}

#[derive(Deserialize, Debug)]
struct Delta {
    content: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Completion {
    choices: Vec<Choice>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a message was provided
    if args.len() < 2 {
        eprintln!("Please provide a message. Usage: {} \"Your message here\"", args[0]);
        std::process::exit(1);
    }

    // Join all arguments after the program name into a single message
    let message = args[1..].join(" ");

    let client = Client::new();
    let response = client
        .post("https://integrate.api.nvidia.com/v1/chat/completions")
        .bearer_auth("$ENTER THE API HERE FROM NVIDIA NIM")
        .json(&json!({
            "model": "nv-mistralai/mistral-nemo-12b-instruct",
            "messages": [{"role": "user", "content": message}],
            "temperature": 0.2,
            "top_p": 0.7,
            "max_tokens": 1024,
            "stream": true
        }))
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        if chunk.starts_with(b"data: ") {
            let data = &chunk[6..];
            if data != b"[DONE]" {
                if let Ok(completion) = serde_json::from_slice::<Completion>(data) {
                    if let Some(content) = &completion.choices[0].delta.content {
                        print!("{}", content);
                    }
                }
            }
        }
    }
    println!(); // Add a newline at the end
    Ok(())
}
  
