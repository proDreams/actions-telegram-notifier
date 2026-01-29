use crate::enums::event_enums::GitHubEvent;
use crate::enums::workflow_enums::{NotifyFields, EventStatus};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DataStructure {
    pub event: GitHubEvent,
    pub token: String,
    pub chat_id: String,
    pub api_url: Option<String>,
    pub thread_id: Option<String>,
    pub status: EventStatus,
    pub title: Option<String>,
    pub message: Option<String>,
    pub footer: Option<String>,
    pub notify_fields: Option<Vec<NotifyFields>>,
    pub workflow: String,
    pub sha: Option<String>,
}
