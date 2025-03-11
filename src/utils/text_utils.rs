use crate::enums::workflow_enums::{NotifyFields, Status};
use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::PushEvent;

#[allow(dead_code)]
pub fn escape_markdown_v2(text: &str) -> String {
    const SPECIAL_CHARS: [char; 18] = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    let mut escaped = String::with_capacity(text.len() * 2);

    for c in text.chars() {
        if SPECIAL_CHARS.contains(&c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }

    escaped
}

pub fn get_input_title(title: &str, status: &Status) -> String {
    if title.is_empty() {
        format!("{} *Workflow status:* {}\n", status.icon(), status.text())
    } else {
        format!("{} {}\n", status.icon(), title)
    }
}

pub fn generate_input_message(message: &str) -> String {
    if !message.is_empty() {
        format!("\n{}\n", message)
    } else {
        "".to_string()
    }
}

pub fn generate_notify_fields(data: &DataStructure, event: &PushEvent) -> String {
    let mut message = String::new();

    for field in data.notify_fields.as_ref().unwrap_or(&Vec::new()) {
        match field {
            NotifyFields::Actor => {
                message.push_str(&format!(
                    "\n🧑‍💻 *Actor:* <a href='{}'>{}</a>",
                    event.sender.html_url, event.sender.login
                ));
            }
            NotifyFields::Repository => {
                message.push_str(&format!(
                    "\n📦 *Repository:* <a href='{}'>{}</a>",
                    event.repository.html_url, event.repository.full_name
                ));
            }
            NotifyFields::Workflow => {
                message.push_str(&format!("\n🏹 *Workflow:* `{}`", data.workflow));
            }
            NotifyFields::Branch => {
                let branch = event.reference.replace("refs/heads/", "");
                message.push_str(&format!("\n🏷️ *Branch:* `{}`", branch));
            }
            NotifyFields::RepoWithTag => {
                let branch = event.reference.replace("refs/heads/", "");
                message.push_str(&format!(
                    "\n🛠️ `@{}:{}`",
                    event.repository.full_name, branch
                ));
            }
            NotifyFields::Commit => {
                message.push_str(&format!("\n🔨 *Commit:* [View Changes]({})", event.compare));
            }
        }
    }

    message.trim_start().to_string()
}
