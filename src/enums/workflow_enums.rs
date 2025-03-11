use serde::Deserialize;

#[derive(Deserialize)]
pub enum Status {
    Success,
    Failure,
    Cancelled,
    Info,
    Pending,
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
