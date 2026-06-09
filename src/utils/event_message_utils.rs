use crate::enums::workflow_enums::EventStatus;
use crate::structures::data_structure::DataStructure;
use crate::structures::event_details::{EventDetails, EventType};
use crate::utils::text_utils::{generate_general_fields, generate_input_message};

pub fn generate_current_message(data: &DataStructure) -> String {
    let details = &data.event_details;

    match details.event_type {
        EventType::Push | EventType::Tag => {
            generate_push_message(data, details)
        }
        EventType::PullRequest => {
            generate_pull_request_message(data, details)
        }
        EventType::PullRequestReview => {
            generate_pull_request_review_message(data, details)
        }
        EventType::WorkflowDispatch | EventType::Pipeline => {
            generate_workflow_dispatch_message(data, details)
        }
    }
}

fn generate_push_message(data: &DataStructure, details: &EventDetails) -> String {
    let mut message = String::new();

    message += &get_push_input_title(data.title.as_deref().unwrap_or_default(), &data.status);

    message += &generate_general_fields(
        &data.notify_fields,
        &details.author_url,
        &details.author_name,
        &details.repo_url,
        &details.repo_name,
        &details.workflow_name,
    );

    message += &generate_push_notify_fields(data, details);

    message += &generate_input_message(data.message.as_deref().unwrap_or_default());
    message += &generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

fn generate_pull_request_message(data: &DataStructure, details: &EventDetails) -> String {
    let mut message = String::new();

    message += &get_pull_request_input_title(
        data.title.as_deref().unwrap_or_default(),
        details.action.as_deref().unwrap_or("opened"),
        details.pr_number.unwrap_or(0),
        details.pr_merged.unwrap_or(false),
    );

    if let (Some(url), Some(title)) = (&details.pr_url, &details.pr_title) {
        message += &format!(
            "📝 <b>PR Title:</b> <a href='{}'>{}</a>\n",
            url, title
        );
    }

    message += &generate_general_fields(
        &data.notify_fields,
        &details.author_url,
        &details.author_name,
        &details.repo_url,
        &details.repo_name,
        &details.workflow_name,
    );

    message += &generate_input_message(data.message.as_deref().unwrap_or_default());
    message += &generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

fn generate_pull_request_review_message(data: &DataStructure, details: &EventDetails) -> String {
    let mut message = String::new();

    let review_state = details.review_state.as_deref().unwrap_or("commented");

    message += &get_pull_request_review_input_title(
        data.title.as_deref().unwrap_or_default(),
        review_state,
    );

    if let (Some(url), Some(title)) = (&details.pr_url, &details.pr_title) {
        message += &format!(
            "📝 <b>PR Title:</b> <a href='{}'>{}</a>\n",
            url, title
        );
    }

    let review_link = details.review_url.as_deref().unwrap_or("");
    if !review_link.is_empty() {
        message += &format!(
            "🔗 <b>Review Link:</b> <a href='{}'>Click here</a>\n",
            review_link
        );
    }

    if let Some(body) = &details.review_body {
        if !body.trim().is_empty() {
            message += &format!(
                "📝 <b>Comment:</b>\n<blockquote>{}</blockquote>\n",
                body
            );
        }
    }

    message += &generate_general_fields(
        &data.notify_fields,
        &details.author_url,
        &details.author_name,
        &details.repo_url,
        &details.repo_name,
        &details.workflow_name,
    );

    message += &generate_input_message(data.message.as_deref().unwrap_or_default());
    message += &generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

fn generate_workflow_dispatch_message(data: &DataStructure, details: &EventDetails) -> String {
    let mut message = String::new();

    message += &get_workflow_dispatch_input_title(data.title.as_deref().unwrap_or_default());

    message += &format!(
        "{} <b>Workflow status:</b> <code>{}</code>\n",
        data.status.icon(),
        data.status.text()
    );

    message += &generate_general_fields(
        &data.notify_fields,
        &details.author_url,
        &details.author_name,
        &details.repo_url,
        &details.repo_name,
        &details.workflow_name,
    );

    message += &generate_workflow_dispatch_notify_fields(data, details);
    message += &generate_workflow_dispatch_inputs_message(&details.workflow_inputs);

    message += &generate_input_message(data.message.as_deref().unwrap_or_default());
    message += &generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

fn get_push_input_title(title: &str, status: &EventStatus) -> String {
    if title.is_empty() {
        format!(
            "{} <b>Workflow status:</b> <code>{}</code>\n",
            status.icon(),
            status.text()
        )
    } else {
        format!("{} <b>{}</b>\n", status.icon(), title)
    }
}

fn generate_push_notify_fields(data: &DataStructure, details: &EventDetails) -> String {
    use crate::enums::workflow_enums::NotifyFields;
    let mut message = String::new();

    for field in data.notify_fields.as_ref().unwrap_or(&Vec::new()) {
        match field {
            NotifyFields::Branch => {
                if let Some(ref branch) = details.branch {
                    message.push_str(&format!("\n🏷️ <b>Branch:</b> <code>{}</code>", branch));
                }
            }
            NotifyFields::RepoWithTag => {
                let branch = details.branch.as_deref().unwrap_or("unknown");
                message.push_str(&format!(
                    "\n🛠️ <code>@{}:{}</code>",
                    details.repo_name, branch
                ));
            }
            NotifyFields::Commit => {
                if let Some(ref msg) = details.commit_message {
                    message.push_str(&format!(
                        "\n🔨 <b>Commit Message:</b> <code>{}</code>",
                        msg.splitn(2, '\n').next().unwrap_or("")
                    ));
                }
            }
            _ => {}
        }
    }

    message
}

fn get_pull_request_input_title(
    title: &str,
    action: &str,
    number: u64,
    merged: bool,
) -> String {
    let (icon, text) = if merged {
        ("🔀", "merged")
    } else {
        let (i, t) = match action {
            "opened" => ("🆕", "opened"),
            "edited" => ("✏️", "edited"),
            "synchronize" => ("🔄", "synchronize"),
            "reopened" => ("🔓", "reopened"),
            "closed" => ("❌", "closed"),
            _ => ("🆕", action),
        };
        (i, t)
    };

    if title.is_empty() {
        format!("{} <b>Pull Request №{}:</b> <code>{}</code>\n", icon, number, text)
    } else {
        format!("{} {}\n", icon, title)
    }
}

fn get_pull_request_review_input_title(title: &str, state: &str) -> String {
    let (icon, default_text) = match state.to_lowercase().as_str() {
        "approved" => ("✅", "Pull Request Approved"),
        "changes_requested" => ("❌", "Changes Requested"),
        "commented" => ("💬", "Review Comment Added"),
        "dismissed" => ("🗑️", "Review Dismissed"),
        _ => ("🔍", "Pull Request Reviewed"),
    };

    if title.is_empty() {
        format!("{} <b>{}</b>\n", icon, default_text)
    } else {
        format!("{} {}\n", icon, title)
    }
}

fn get_workflow_dispatch_input_title(title: &str) -> String {
    if title.is_empty() {
        "🚀 <b>Workflow Dispatched</b>\n".to_string()
    } else {
        format!("🚀 {}\n", title)
    }
}

fn generate_workflow_dispatch_notify_fields(
    data: &DataStructure,
    details: &EventDetails,
) -> String {
    use crate::enums::workflow_enums::NotifyFields;
    let mut message = String::new();

    for field in data.notify_fields.as_ref().unwrap_or(&Vec::new()) {
        match field {
            NotifyFields::Branch => {
                if let Some(ref branch) = details.branch {
                    message.push_str(&format!("\n🏷️ <b>Branch:</b> <code>{}</code>", branch));
                }
            }
            NotifyFields::RepoWithTag => {
                let branch = details.branch.as_deref().unwrap_or("unknown");
                message.push_str(&format!(
                    "\n🛠️ <code>@{}:{}</code>",
                    details.repo_name, branch
                ));
            }
            NotifyFields::Commit => {
                if let Some(ref sha) = details.commit_sha {
                    let short_sha: String = sha.chars().take(7).collect();
                    if !short_sha.is_empty() {
                        message.push_str(&format!("\n🔨 <b>Commit:</b> <code>{}</code>", short_sha));
                    }
                }
            }
            _ => {}
        }
    }

    message
}

fn generate_workflow_dispatch_inputs_message(inputs: &[(String, String)]) -> String {
    if inputs.is_empty() {
        return String::new();
    }

    let inputs_str = inputs
        .iter()
        .map(|(key, value)| {
            if value.is_empty() {
                format!("- {}", key)
            } else {
                format!("- {}: {}", key, value)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "\n\n🔧 <b>With custom inputs:</b>\n<blockquote>{}</blockquote>\n",
        inputs_str
    )
}
