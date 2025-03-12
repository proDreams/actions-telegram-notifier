use crate::enums::workflow_enums::{NotifyFields, PullRequestAction, PushStatus};
use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::PushEvent;
use crate::structures::event_type_structures::pull_request_structures::PullRequestData;

pub fn get_push_input_title(title: &str, status: &PushStatus) -> String {
    if title.is_empty() {
        format!(
            "{} <b>Workflow status:</b> <code>{}</code>\n",
            status.icon(),
            status.text()
        )
    } else {
        format!("{} {}\n", status.icon(), title)
    }
}

pub fn get_pull_request_input_title(
    title: &str,
    action: &PullRequestAction,
    number: &u64,
    merged: &bool,
) -> String {
    if title.is_empty() {
        if *merged {
            format!(
                "{} <b>Pull Request №{}:</b> <code>{}</code>\n",
                action.merged_icon(),
                number,
                action.merged_text()
            )
        } else {
            format!(
                "{} <b>Pull Request №{}:</b> <code>{}</code>\n",
                action.icon(),
                number,
                action.text()
            )
        }
    } else {
        format!("{} {}\n", action.icon(), title)
    }
}

pub fn get_pull_request_title(data: &PullRequestData) -> String {
    format!(
        "📝 <b>PR Title:</b> <a href='{}'>{}</a>\n",
        data.html_url, data.title
    )
}

pub fn generate_input_message(message: &str) -> String {
    if !message.is_empty() {
        format!("\n{}\n", message)
    } else {
        "".to_string()
    }
}

pub fn generate_general_fields(
    notify_fields: &Option<Vec<NotifyFields>>,
    sender_html_url: &String,
    login: &String,
    repository_html_url: &String,
    full_name: &String,
    workflow: &String,
) -> String {
    let mut message: String = String::new();

    for field in notify_fields.as_ref().unwrap_or(&Vec::new()) {
        match field {
            NotifyFields::Actor => {
                message.push_str(&format!(
                    "\n🧑‍💻 <b>Actor:</b> <a href='{}'>{}</a>",
                    sender_html_url, login
                ));
            }
            NotifyFields::Repository => {
                message.push_str(&format!(
                    "\n📦 <b>Repository:</b> <a href='{}'>{}</a>",
                    repository_html_url, full_name
                ));
            }
            NotifyFields::Workflow => {
                message.push_str(&format!("\n🏹 <b>Workflow:</b> <code>{}</code>", workflow));
            }
            _ => {}
        }
    }

    message.to_string()
}

pub fn generate_push_notify_fields(data: &DataStructure, event: &PushEvent) -> String {
    let mut message: String = String::new();

    for field in data.notify_fields.as_ref().unwrap_or(&Vec::new()) {
        match field {
            NotifyFields::Branch => {
                let branch = event.reference.replace("refs/heads/", "");
                message.push_str(&format!("\n🏷️ <b>Branch:</b> <code>{}</code>", branch));
            }
            NotifyFields::RepoWithTag => {
                let branch = event.reference.replace("refs/heads/", "");
                message.push_str(&format!(
                    "\n🛠️ <code>@{}:{}</code>",
                    event.repository.full_name, branch
                ));
            }
            NotifyFields::Commit => {
                message.push_str(&format!(
                    "\n🔨 <b>Commit Message:</b> <code>{}</code>",
                    event
                        .head_commit
                        .message
                        .splitn(2, '\n')
                        .next()
                        .unwrap_or("")
                ));
            }
            _ => {}
        }
    }

    message.to_string()
}
