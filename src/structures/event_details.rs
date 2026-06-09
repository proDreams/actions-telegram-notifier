#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Push,
    PullRequest,
    PullRequestReview,
    WorkflowDispatch,
    Tag,
    Pipeline,
}

#[derive(Debug, Clone)]
pub struct EventDetails {
    pub event_type: EventType,

    pub author_name: String,
    pub author_url: String,
    pub repo_name: String,
    pub repo_url: String,
    pub workflow_name: String,
    pub branch: Option<String>,
    pub commit_sha: Option<String>,
    pub commit_message: Option<String>,

    pub compare_url: Option<String>,

    pub action: Option<String>,
    pub pr_number: Option<u64>,
    pub pr_title: Option<String>,
    pub pr_url: Option<String>,
    pub pr_merged: Option<bool>,

    pub review_state: Option<String>,
    pub review_url: Option<String>,
    pub review_body: Option<String>,

    pub workflow_inputs: Vec<(String, String)>,

    pub keyboard_url: Option<String>,
    pub keyboard_text: Option<String>,
}

impl EventDetails {
    pub(crate) fn compute_keyboard(&mut self) {
        match self.event_type {
            EventType::Push => {
                if let Some(ref url) = self.compare_url {
                    self.keyboard_url = Some(url.clone());
                    self.keyboard_text = Some("↗️ Link to commit".into());
                }
            }
            EventType::PullRequest => {
                if let Some(ref url) = self.pr_url {
                    self.keyboard_url = Some(url.clone());
                    self.keyboard_text = Some("↗️ Link to Pull Request".into());
                }
            }
            EventType::PullRequestReview => {
                let url = self.review_url.as_ref()
                    .filter(|u| !u.is_empty())
                    .or(self.pr_url.as_ref())
                    .cloned();
                if let Some(ref u) = url {
                    self.keyboard_url = Some(u.clone());
                    self.keyboard_text = Some("↗️ Link to Review".into());
                }
            }
            EventType::WorkflowDispatch => {
                let branch = self.branch.as_deref().unwrap_or("main");
                let url = format!(
                    "{}/blob/{}/{}",
                    self.repo_url.trim_end_matches('/'),
                    branch,
                    self.workflow_name
                );
                self.keyboard_url = Some(url);
                self.keyboard_text = Some("↗️ Link to Workflow".into());
            }
            EventType::Tag | EventType::Pipeline => {
                self.keyboard_url = Some(self.repo_url.clone());
                self.keyboard_text = Some("↗️ Open Repository".into());
            }
        }
    }
}
