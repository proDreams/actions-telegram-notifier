use crate::structures::event_structures::{PullRequestEvent, PushEvent, WorkflowDispatchEvent};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum GitHubEvent {
    Push(PushEvent),
    PullRequest(PullRequestEvent),
    WorkflowDispatch(WorkflowDispatchEvent)
}
