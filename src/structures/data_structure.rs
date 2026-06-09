use crate::enums::workflow_enums::{NotifyFields, EventStatus};
use crate::structures::event_details::EventDetails;

pub struct DataStructure {
    pub token: String,
    pub chat_id: String,
    pub api_url: Option<String>,
    pub thread_id: Option<String>,
    pub status: EventStatus,
    pub title: Option<String>,
    pub message: Option<String>,
    pub footer: Option<String>,
    pub notify_fields: Option<Vec<NotifyFields>>,
    pub proxy_url: Option<String>,
    pub event_details: EventDetails,
}
