use crate::enums::event_enums::GitHubEvent;
use crate::structures::event_details::{EventDetails, EventType};
use crate::structures::event_structures::{
    PullRequestEvent, PullRequestReviewEvent, PushEvent, WorkflowDispatchEvent,
};

impl EventDetails {
    pub fn from_github_event(event: &GitHubEvent, workflow: &str, sha: &Option<String>) -> Self {
        let mut details: EventDetails = match event {
            GitHubEvent::Push(e) => {
                let mut d = EventDetails::from(e);
                d.commit_sha = sha.clone();
                d.workflow_name = workflow.to_string();
                d
            }
            GitHubEvent::PullRequest(e) => {
                let mut d = EventDetails::from(e);
                d.workflow_name = workflow.to_string();
                d
            }
            GitHubEvent::PullRequestReview(e) => {
                let mut d = EventDetails::from(e);
                d.workflow_name = workflow.to_string();
                d
            }
            GitHubEvent::WorkflowDispatch(e) => {
                let mut d = EventDetails::from(e);
                d.commit_sha = sha.clone();
                d
            }
        };
        details.compute_keyboard();
        details
    }
}

impl From<&PushEvent> for EventDetails {
    fn from(event: &PushEvent) -> Self {
        let branch = event
            .reference
            .strip_prefix("refs/heads/")
            .or_else(|| {
                event
                    .reference
                    .strip_prefix("refs/tags/")
            })
            .map(|s| s.to_string())
            .or_else(|| {
                if event.reference.starts_with("refs/") {
                    None
                } else {
                    Some(event.reference.clone())
                }
            });

        let commit_message = event
            .head_commit
            .message
            .splitn(2, '\n')
            .next()
            .unwrap_or("")
            .to_string();

        let compare_url = event
            .get_compare()
            .filter(|u| !u.is_empty());

        let mut details = EventDetails {
            event_type: EventType::Push,
            author_name: event.sender.login.clone(),
            author_url: event.sender.html_url.clone(),
            repo_name: event.repository.full_name.clone(),
            repo_url: event.repository.html_url.clone(),
            workflow_name: String::new(),
            branch,
            commit_sha: None,
            commit_message: Some(commit_message),
            compare_url,
            action: None,
            pr_number: None,
            pr_title: None,
            pr_url: None,
            pr_merged: None,
            review_state: None,
            review_url: None,
            review_body: None,
            workflow_inputs: Vec::new(),
            keyboard_url: None,
            keyboard_text: None,
        };
        details.compute_keyboard();
        details
    }
}

impl From<&PullRequestEvent> for EventDetails {
    fn from(event: &PullRequestEvent) -> Self {
        let action_str = if event.pull_request.merged {
            "merged".to_string()
        } else {
            event.action.text().to_string()
        };

        let mut details = EventDetails {
            event_type: EventType::PullRequest,
            author_name: event.sender.login.clone(),
            author_url: event.sender.html_url.clone(),
            repo_name: event.repository.full_name.clone(),
            repo_url: event.repository.html_url.clone(),
            workflow_name: String::new(),
            branch: None,
            commit_sha: None,
            commit_message: None,
            compare_url: None,
            action: Some(action_str),
            pr_number: Some(event.number),
            pr_title: Some(event.pull_request.title.clone()),
            pr_url: Some(event.pull_request.html_url.clone()),
            pr_merged: Some(event.pull_request.merged),
            review_state: None,
            review_url: None,
            review_body: None,
            workflow_inputs: Vec::new(),
            keyboard_url: None,
            keyboard_text: None,
        };
        details.compute_keyboard();
        details
    }
}

impl From<&PullRequestReviewEvent> for EventDetails {
    fn from(event: &PullRequestReviewEvent) -> Self {
        let review_state = if event.review.state.is_empty() {
            Some(event.action.clone())
        } else {
            Some(event.review.state.clone())
        };

        let review_url = if event.review.html_url.is_empty() {
            Some(event.pull_request.html_url.clone())
        } else {
            Some(event.review.html_url.clone())
        };

        let review_body = event
            .review
            .body
            .as_ref()
            .filter(|b| !b.trim().is_empty())
            .map(|b| {
                if b.chars().count() > 150 {
                    format!("{}...", b.chars().take(150).collect::<String>())
                } else {
                    b.clone()
                }
            });

        let mut details = EventDetails {
            event_type: EventType::PullRequestReview,
            author_name: event.sender.login.clone(),
            author_url: event.sender.html_url.clone(),
            repo_name: event.repository.full_name.clone(),
            repo_url: event.repository.html_url.clone(),
            workflow_name: String::new(),
            branch: None,
            commit_sha: None,
            commit_message: None,
            compare_url: None,
            action: None,
            pr_number: None,
            pr_title: Some(event.pull_request.title.clone()),
            pr_url: Some(event.pull_request.html_url.clone()),
            pr_merged: None,
            review_state,
            review_url,
            review_body,
            workflow_inputs: Vec::new(),
            keyboard_url: None,
            keyboard_text: None,
        };
        details.compute_keyboard();
        details
    }
}

impl From<&WorkflowDispatchEvent> for EventDetails {
    fn from(event: &WorkflowDispatchEvent) -> Self {
        let branch = event
            .reference
            .strip_prefix("refs/heads/")
            .map(|s| s.to_string())
            .or_else(|| Some(event.reference.clone()));

        let mut details = EventDetails {
            event_type: EventType::WorkflowDispatch,
            author_name: event.sender.login.clone(),
            author_url: event.sender.html_url.clone(),
            repo_name: event.repository.full_name.clone(),
            repo_url: event.repository.html_url.clone(),
            workflow_name: event.workflow.clone(),
            branch,
            commit_sha: None,
            commit_message: None,
            compare_url: None,
            action: None,
            pr_number: None,
            pr_title: None,
            pr_url: None,
            pr_merged: None,
            review_state: None,
            review_url: None,
            review_body: None,
            workflow_inputs: event.inputs.clone(),
            keyboard_url: None,
            keyboard_text: None,
        };
        details.compute_keyboard();
        details
    }
}
