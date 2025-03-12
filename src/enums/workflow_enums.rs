use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum PushStatus {
    Success,
    Failure,
    Cancelled,
    Info,
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestAction {
    Opened,
    Edited,
    Synchronize,
    Reopened,
    Closed,
}

#[derive(Deserialize)]
pub enum NotifyFields {
    Actor,
    Repository,
    Workflow,
    Branch,
    Commit,
    RepoWithTag,
}
