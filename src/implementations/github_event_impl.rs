use std::error::Error;
use crate::enums::event_enums::GitHubEvent;
use crate::structures::event_structures::{PullRequestEvent, PushEvent, WorkflowDispatchEvent};

impl GitHubEvent {
    pub fn from_str(event_name: &str, event_json: &str) -> Result<Self, Box<dyn Error>> {
        match event_name {
            "push" => {
                let event: PushEvent = serde_json::from_str(event_json)?;
                Ok(GitHubEvent::Push(event))
            }
            "pull_request" => {
                let event: PullRequestEvent = serde_json::from_str(event_json)?;
                Ok(GitHubEvent::PullRequest(event))
            }
            "workflow_dispatch" => {
                let event: WorkflowDispatchEvent = serde_json::from_str(event_json)?;
                Ok(GitHubEvent::WorkflowDispatch(event))
            }
            _ => Err(format!("Unknown event: {}", event_name).into()),
        }
    }
}
