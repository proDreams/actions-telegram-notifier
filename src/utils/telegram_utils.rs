use crate::enums::event_enums::GitHubEvent;
use crate::structures::data_structure::DataStructure;
use serde_json::json;

fn generate_keyboard(github_event: &GitHubEvent) -> serde_json::Value {
    match &github_event {
        GitHubEvent::Push(event) => {
            json!({
                "text": "↗️ Link to commit",
                "url": event.get_compare()
            })
        }
        GitHubEvent::PullRequest(event) => {
            json!({
                "text": "↗️ Link to Pull Request",
                "url": event.pull_request.html_url
            })
        }
        GitHubEvent::WorkflowDispatch(event) => {
            json!({
                "text": "↗️ Link to Workflow",
                "url": format!("{}/blob/{}/{}", event.repository.html_url, event.reference.replace("refs/heads/", ""), event.workflow)
            })
        }
    }
}

fn generate_json(message_text: &String, data: &DataStructure) -> serde_json::value::Value {
    json!({
        "chat_id": data.chat_id,
        "message_thread_id": data.thread_id,
        "text": message_text,
        "parse_mode": "HTML",
        "disable_web_page_preview": true,
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
    let api_url_base = data
        .api_url
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("https://api.telegram.org");
    let api_url = format!("{}/bot{}/sendMessage", api_url_base, data.token);

    let json = generate_json(&message_text, &data);

    let mut client_builder = reqwest::Client::builder();

    if let Some(proxy_str) = &data.proxy_url {
        if !proxy_str.is_empty() {
            match reqwest::Proxy::all(proxy_str) {
                Ok(proxy) => {
                    client_builder = client_builder.proxy(proxy);
                    println!("🔧 Using proxy for connection...");
                }
                Err(e) => eprintln!("⚠️ Warning: Failed to parse proxy URL, ignoring it: {}", e),
            }
        }
    }

    let client = client_builder.build().expect("Failed to build HTTP client");

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
