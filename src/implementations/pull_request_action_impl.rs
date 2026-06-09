use crate::enums::workflow_enums::PullRequestAction;

impl PullRequestAction {
    pub fn text(&self) -> &str {
        match self {
            PullRequestAction::Opened => "opened",
            PullRequestAction::Edited => "edited",
            PullRequestAction::Synchronize => "synchronize",
            PullRequestAction::Reopened => "reopened",
            PullRequestAction::Closed => "closed",
        }
    }
}
