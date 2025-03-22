use crate::enums::workflow_enums::{NotifyFields};


pub fn generate_input_message(message: &str) -> String {
    if !message.is_empty() {
        format!("\n\n{}", message)
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
