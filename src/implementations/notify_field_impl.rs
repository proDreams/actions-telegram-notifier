use crate::enums::workflow_enums::{NotifyFields};

impl NotifyFields {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "actor" => Some(NotifyFields::Actor),
            "repository" => Some(NotifyFields::Repository),
            "workflow" => Some(NotifyFields::Workflow),
            "branch" => Some(NotifyFields::Branch),
            "commit" => Some(NotifyFields::Commit),
            "repo_with_tag" => Some(NotifyFields::RepoWithTag),
            _ => None,
        }
    }
}
