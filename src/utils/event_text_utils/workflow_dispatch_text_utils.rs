use crate::enums::workflow_enums::EventStatus;
use crate::enums::workflow_enums::NotifyFields;
use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::WorkflowDispatchEvent;

pub fn get_workflow_dispatch_input_title(title: &str) -> String {
    if title.is_empty() {
        "🚀 <b>Workflow Dispatched</b>\n".to_string()
    } else {
        format!("🚀 {}\n", title)
    }
}

pub fn get_workflow_dispatch_status(status: &EventStatus) -> String {
    format!(
        "{} <b>Workflow status:</b> <code>{}</code>\n",
        status.icon(),
        status.text()
    )
}

pub fn generate_workflow_dispatch_inputs_message(inputs: &Vec<(String, String)>) -> String {
    let inputs_str = if inputs.is_empty() {
        return "".to_string();
    } else {
        inputs
            .iter()
            .map(|(key, value)| {
                if value.is_empty() {
                    format!("- {}", key)
                } else {
                    format!("- {}: {}", key, value)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "\n\n🔧 <b>With custom inputs:</b>\n<blockquote>{}</blockquote>\n",
        inputs_str
    )
}

pub fn generate_workflow_dispatch_notify_fields(
    data: &DataStructure,
    event: &WorkflowDispatchEvent,
) -> String {
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
                let short_sha = data
                    .sha
                    .as_deref()
                    .unwrap_or("")
                    .chars()
                    .take(7)
                    .collect::<String>();
                if !short_sha.is_empty() {
                    message.push_str(&format!(
                        "\n🔨 <b>Commit:</b> <code>{}</code>",
                        short_sha
                    ));
                }
            }
            _ => {}
        }
    }

    message
}
