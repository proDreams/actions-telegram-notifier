use crate::enums::workflow_enums::EventStatus;

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

pub fn generate_workflow_dispatch_inputs_message(inputs: &Vec<String>) -> String {
    let inputs_str = if inputs.is_empty() {
        return "".to_string()
    } else {
        inputs.iter()
            .map(|key| format!("- {}", key))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "\n\n🔧 <b>With custom inputs:</b>\n<blockquote>{}</blockquote>\n",
        inputs_str
    )
}
