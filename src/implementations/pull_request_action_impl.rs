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

    pub fn icon(&self) -> &str {
        match self {
            PullRequestAction::Opened => "🆕",
            PullRequestAction::Edited => "✏️",
            PullRequestAction::Synchronize => "🔄",
            PullRequestAction::Reopened => "🔓",
            PullRequestAction::Closed => "❌",
        }
    }
}
