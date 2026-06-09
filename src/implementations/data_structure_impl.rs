use crate::enums::workflow_enums::{NotifyFields, EventStatus};
use crate::implementations::gitlab_event_impl;
use crate::structures::data_structure::DataStructure;
use crate::structures::event_details::EventDetails;
use crate::utils::env_utils::get_env_var;
use std::error::Error;
use std::fs;

impl DataStructure {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let token = get_env_var("INPUT_TOKEN")?;
        let chat_id = get_env_var("INPUT_CHAT_ID")?;

        let api_url = get_env_var("INPUT_API_URL").ok();
        let thread_id = get_env_var("INPUT_THREAD_ID").ok();
        let title = get_env_var("INPUT_TITLE").ok();
        let message = get_env_var("INPUT_MESSAGE").ok();
        let footer = get_env_var("INPUT_FOOTER").ok();

        let proxy_url = get_env_var("INPUT_PROXY_URL").ok();

        let status_input = get_env_var("INPUT_STATUS").ok();
        let status = EventStatus::from_str(status_input.as_deref().unwrap_or("info"));

        let notify_fields = get_env_var("INPUT_NOTIFY_FIELDS")
            .ok()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .filter_map(NotifyFields::from_str)
                    .collect::<Vec<_>>()
            });

        // --- Provider detection ---
        let is_gitlab = std::env::var("GITLAB_CI").ok().as_deref() == Some("true");

        if is_gitlab {
            return Self::from_gitlab(
                token, chat_id, api_url, thread_id, title, message,
                footer, proxy_url, status, notify_fields,
            );
        }

        // --- GitHub mode (existing logic) ---
        use crate::enums::event_enums::GitHubEvent;
        let event_name = get_env_var("GITHUB_EVENT_NAME")?;
        let event_path = get_env_var("GITHUB_EVENT_PATH")?;
        let event_json = fs::read_to_string(event_path)?;
        let event = GitHubEvent::from_str(&event_name, &event_json)?;

        let workflow = get_env_var("GITHUB_WORKFLOW")?;
        let sha = get_env_var("GITHUB_SHA").ok();

        let event_details = EventDetails::from_github_event(&event, &workflow, &sha);

        Ok(DataStructure {
            token,
            chat_id,
            api_url,
            thread_id,
            status,
            title,
            message,
            footer,
            notify_fields,
            proxy_url,
            event_details,
        })
    }

    fn from_gitlab(
        token: String, chat_id: String,
        api_url: Option<String>, thread_id: Option<String>,
        title: Option<String>, message: Option<String>,
        footer: Option<String>, proxy_url: Option<String>,
        status: EventStatus, notify_fields: Option<Vec<NotifyFields>>,
    ) -> Result<Self, Box<dyn Error>> {
        let event_details = gitlab_event_impl::parse_gitlab_event()
            .ok_or_else(|| "Failed to parse GitLab CI event from environment variables".to_string())?;

        Ok(DataStructure {
            token,
            chat_id,
            api_url,
            thread_id,
            status,
            title,
            message,
            footer,
            notify_fields,
            proxy_url,
            event_details,
        })
    }
}
