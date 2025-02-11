use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::env;

fn get_env_values() -> Result<(HashMap<String, String>, Vec<String>), String> {
    let keys = [
        "INPUT_TOKEN",
        "INPUT_CHAT_ID",
        "INPUT_THREAD_ID",
        "INPUT_STATUS",
        "INPUT_TITLE",
        "INPUT_MESSAGE",
        "INPUT_FOOTER",
        "GITHUB_ACTOR",
        "GITHUB_SERVER_URL",
        "GITHUB_WORKFLOW",
        "GITHUB_REPOSITORY",
        "GITHUB_REF_NAME",
        "GITHUB_SHA",
        "GITHUB_EVENT_PATH",
    ];

    let status_icons: HashMap<String, &str> = HashMap::from([
        ("success".to_string(), "✅"),
        ("failure".to_string(), "🔴"),
        ("cancelled".to_string(), "❌"),
        ("info".to_string(), "🔔"),
    ]);

    let mut env_values: HashMap<String, String> = HashMap::new();

    for &key in &keys {
        let value = env::var(key).unwrap_or_default();
        env_values.insert(key.to_string(), value);
    }

    if let Ok(event_path) = env::var("GITHUB_EVENT_PATH") {
        if let Ok(event_content) = std::fs::read_to_string(&event_path) {
            if let Ok(event_json) = serde_json::from_str::<serde_json::Value>(&event_content) {
                if let Some(commit_message) = event_json["head_commit"]["message"].as_str() {
                    env_values.insert("COMMIT_MESSAGE".to_string(), commit_message.to_string());
                }
            }
        }
    }
    
    if env_values["INPUT_TOKEN"].is_empty() {
        return Err("Missing required token!".to_string());
    }
    if env_values["INPUT_CHAT_ID"].is_empty() {
        return Err("Missing required chat_id!".to_string());
    }

    let status = if env_values["INPUT_STATUS"].is_empty()
        || !status_icons.contains_key(&env_values["INPUT_STATUS"])
    {
        eprintln!("Status not provided or invalid! Setting to default \"info\" ");
        "info".to_string()
    } else {
        env_values["INPUT_STATUS"].clone()
    };

    env_values.insert("INPUT_STATUS".to_string(), status.clone());
    env_values.insert(
        "INPUT_STATUS_ICON".to_string(),
        status_icons.get(&status).unwrap().to_string(),
    );

    let mut notify_fields_vec: Vec<String> = Vec::new();

    if let Ok(notify_fields) = env::var("INPUT_NOTIFY_FIELDS") {
        if !notify_fields.is_empty() {
            notify_fields_vec = notify_fields.split(',').map(|s| s.to_string()).collect();
        }
    }

    Ok((env_values, notify_fields_vec))
}

fn gen_notify_message(env_values: &HashMap<String, String>, notify_fields: Vec<String>) -> String {
    let mut message = "".to_owned();

    if env_values["INPUT_TITLE"].is_empty() {
        message += format!(
            "{} *Workflow status:* {}\n",
            env_values["INPUT_STATUS_ICON"], env_values["INPUT_STATUS"]
        )
        .as_str();
    } else {
        message += format!(
            "{} {} *{}*\n",
            env_values["INPUT_STATUS_ICON"], env_values["INPUT_TITLE"], env_values["INPUT_STATUS"]
        )
        .as_str();
    }

    if !notify_fields.is_empty() {
        if notify_fields.contains(&"actor".to_string()) {
            message += format!("🧑‍💻 *Actor:* {}\n", env_values["GITHUB_ACTOR"]).as_str();
        }
        if notify_fields.contains(&"repository".to_string()) {
            message += format!("📦 *Repository:* {}\n", env_values["GITHUB_REPOSITORY"]).as_str();
        }
        if notify_fields.contains(&"workflow".to_string()) {
            message += format!("🚀 *Workflow:* {}\n", env_values["GITHUB_WORKFLOW"]).as_str();
        }
        if notify_fields.contains(&"branch".to_string()) {
            message += format!("🚀 *Branch:* {}\n", env_values["GITHUB_REF_NAME"]).as_str();
        }
        if notify_fields.contains(&"commit".to_string()) {
            message += format!("🚀 *Commit Message:* {}\n", env_values["COMMIT_MESSAGE"]).as_str();
        }
    }

    if !env_values["INPUT_MESSAGE"].is_empty() {
        message += format!("\n{}\n", env_values["INPUT_MESSAGE"]).as_str();
    }

    if !env_values["INPUT_FOOTER"].is_empty() {
        message += format!("\n{}\n", env_values["INPUT_FOOTER"]).as_str();
    }

    message
}

async fn send_notify_message(message_text: String, env_values: &HashMap<String, String>) {
    let api_url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        env_values["INPUT_TOKEN"]
    );

    let mut data = json!({
        "chat_id": env_values["INPUT_CHAT_ID"],
        "text": escape_markdown_v2(&message_text),
        "parse_mode": "MarkdownV2",
        "reply_markup": {
            "inline_keyboard": [
                [
                    {
                        "text": "↗️ Link to commit",
                        "url": format!("{}/{}/commit/{}", env_values["GITHUB_SERVER_URL"], env_values["GITHUB_REPOSITORY"], env_values["GITHUB_SHA"])
                    }
                ]
            ]
        }
    });

    if !env_values["INPUT_THREAD_ID"].is_empty() {
        if let Some(data_obj) = data.as_object_mut() {
            data_obj.insert(
                "message_thread_id".to_string(),
                serde_json::Value::String(env_values["INPUT_THREAD_ID"].clone()),
            );
        }
    }

    let client = Client::new();
    let response = client
        .post(&api_url)
        .json(&data)
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

fn escape_markdown_v2(text: &str) -> String {
    text.replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('~', "\\~")
        .replace('`', "\\`")
        .replace('>', "\\>")
        .replace('#', "\\#")
        .replace('+', "\\+")
        .replace('-', "\\-")
        .replace('=', "\\=")
        .replace('|', "\\|")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('.', "\\.")
        .replace('!', "\\!")
        .replace('_', "\\_")
}

#[tokio::main]
async fn main() {
    let (env_values, notify_fields_vec) = match get_env_values() {
        Ok((env_values, notify_fields_vec)) => (env_values, notify_fields_vec),
        Err(err) => panic!("{err}"),
    };

    let message = gen_notify_message(&env_values, notify_fields_vec);
    println!("{message}");
    send_notify_message(message, &env_values).await;
}
