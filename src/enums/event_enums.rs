use crate::structures::event_structures::{PullRequestEvent, PushEvent};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum GitHubEvent {
    Push(PushEvent),
    PullRequest(PullRequestEvent),
}
