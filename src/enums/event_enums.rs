use crate::structures::event_structures::{PullRequestEvent, PullRequestReviewEvent, PushEvent, WorkflowDispatchEvent};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum GitHubEvent {
    Push(PushEvent),
    PullRequest(PullRequestEvent),
    WorkflowDispatch(WorkflowDispatchEvent),
    PullRequestReview(PullRequestReviewEvent),
}
