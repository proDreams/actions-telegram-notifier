use crate::enums::workflow_enums::{NotifyFields, EventStatus};
use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::PushEvent;

pub fn get_push_input_title(title: &str, status: &EventStatus) -> String {
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
