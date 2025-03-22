use crate::enums::workflow_enums::PullRequestAction;
use crate::structures::event_type_structures::pull_request_structures::PullRequestData;

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