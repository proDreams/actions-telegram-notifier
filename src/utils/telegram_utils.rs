use crate::enums::event_enums::GitHubEvent;
use crate::structures::data_structure::DataStructure;
use reqwest::Client;
use serde_json::json;

fn generate_keyboard(github_event: &GitHubEvent) -> serde_json::Value {
    match &github_event {
        GitHubEvent::Push(event) => {
            json!({
                "text": "↗️ Link to commit",
                "url": event.compare.clone()
            })
        }
        GitHubEvent::PullRequest(event) => {
            json!({
                "text": "↗️ Link to commit",
                "url": "https://api.github.com" // temporary link
            })
        }
    }
}

fn generate_json(message_text: &String, data: &DataStructure) -> serde_json::value::Value {
    json!({
        "chat_id": data.chat_id,
        "message_thread_id": data.thread_id,
        "text": message_text,
        "parse_mode": "MarkdownV2",
        "reply_markup": {
            "inline_keyboard": [
                [
                    generate_keyboard(&data.event)
                ]
            ]
        }
    })
}

pub async fn send_notify_message(message_text: String, data: &DataStructure) {
    let api_url = format!("https://api.telegram.org/bot{}/sendMessage", data.token);

    let json = generate_json(&message_text, &data);

    let client = Client::new();
    let response = client
        .post(&api_url)
        .json(&json)
        .send()
        .await
        .expect("Failed to send request");

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.expect("Failed to read response body");
        eprintln!("Error: {}\nResponse Body: {}", status, error_text);
    } else {
        println!("✨ Notify send successful!");
    }
}
