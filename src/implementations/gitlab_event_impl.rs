use crate::structures::event_details::{EventDetails, EventType};
use std::env;

pub fn parse_gitlab_event() -> Option<EventDetails> {
    let pipeline_source = env::var("CI_PIPELINE_SOURCE").ok()?;

    let repo_url = env::var("CI_PROJECT_URL").unwrap_or_default();
    let repo_path = env::var("CI_PROJECT_PATH").unwrap_or_default();
    let author_name = env::var("CI_COMMIT_AUTHOR")
        .unwrap_or_else(|_| "unknown".to_string());
    let commit_sha = env::var("CI_COMMIT_SHA").ok();
    let commit_message = env::var("CI_COMMIT_MESSAGE").ok()
        .map(|m| m.splitn(2, '\n').next().unwrap_or("").to_string());
    let workflow_name = env::var("CI_JOB_NAME").unwrap_or_default();

    let author_url = if !repo_url.is_empty() && !author_name.is_empty() {
        format!("{}/-/profile", repo_url.trim_end_matches('/'))
    } else {
        String::new()
    };

    match pipeline_source.as_str() {
        "push" => parse_gitlab_push(
            &repo_url, &repo_path, &author_name, &author_url,
            &commit_sha, &commit_message, &workflow_name,
        ),
        "merge_request_event" => parse_gitlab_merge_request(
            &repo_url, &repo_path, &author_name, &author_url,
            &commit_sha, &workflow_name,
        ),
        "web" | "api" | "schedule" | "trigger" | "pipeline" | "chat" => {
            parse_gitlab_pipeline(
                &repo_url, &repo_path, &author_name, &author_url,
                &commit_sha, &commit_message, &workflow_name, &pipeline_source,
            )
        }
        _ => {
            Some(generic_gitlab_event(
                EventType::Pipeline,
                &repo_url, &repo_path, &author_name, &author_url,
                &workflow_name,
            ))
        }
    }
}

fn parse_gitlab_push(
    repo_url: &str, repo_path: &str, author_name: &str, author_url: &str,
    commit_sha: &Option<String>, commit_message: &Option<String>, workflow_name: &str,
) -> Option<EventDetails> {
    let branch = env::var("CI_COMMIT_BRANCH").ok();
    let tag = env::var("CI_COMMIT_TAG").ok();
    let event_type = if tag.is_some() { EventType::Tag } else { EventType::Push };

    let compare_url = if let (Some(sha), Some(br)) = (commit_sha.as_ref(), branch.as_ref()) {
        Some(format!(
            "{}/-/compare/{}...{}?from=main&to={}",
            repo_url.trim_end_matches('/'),
            sha,
            sha,
            br
        ))
    } else {
        commit_sha.as_ref().map(|sha| {
            format!("{}/-/commit/{}", repo_url.trim_end_matches('/'), sha)
        })
    };

    let mut details = EventDetails {
        event_type,
        author_name: author_name.to_string(),
        author_url: author_url.to_string(),
        repo_name: repo_path.to_string(),
        repo_url: repo_url.to_string(),
        workflow_name: workflow_name.to_string(),
        branch,
        commit_sha: commit_sha.clone(),
        commit_message: commit_message.clone(),
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
    Some(details)
}

fn parse_gitlab_merge_request(
    repo_url: &str, repo_path: &str, author_name: &str, author_url: &str,
    commit_sha: &Option<String>, workflow_name: &str,
) -> Option<EventDetails> {
    let iid = env::var("CI_MERGE_REQUEST_IID").ok()
        .and_then(|v| v.parse::<u64>().ok());
    let title = env::var("CI_MERGE_REQUEST_TITLE").ok();
    let source_branch = env::var("CI_MERGE_REQUEST_SOURCE_BRANCH_NAME").ok();
    let _target_branch = env::var("CI_MERGE_REQUEST_TARGET_BRANCH_NAME").ok();

    let mr_event_type = env::var("CI_MERGE_REQUEST_EVENT_TYPE").ok();
    let is_merged = mr_event_type.as_deref() == Some("merged_result");

    let action = if is_merged {
        "merged"
    } else {
        "opened"
    };

    let pr_url = iid.map(|num| {
        format!("{}/-/merge_requests/{}", repo_url.trim_end_matches('/'), num)
    });

    let mut details = EventDetails {
        event_type: EventType::PullRequest,
        author_name: author_name.to_string(),
        author_url: author_url.to_string(),
        repo_name: repo_path.to_string(),
        repo_url: repo_url.to_string(),
        workflow_name: workflow_name.to_string(),
        branch: source_branch,
        commit_sha: commit_sha.clone(),
        commit_message: None,
        compare_url: None,
        action: Some(action.to_string()),
        pr_number: iid,
        pr_title: title,
        pr_url,
        pr_merged: Some(is_merged),
        review_state: None,
        review_url: None,
        review_body: None,
        workflow_inputs: Vec::new(),
        keyboard_url: None,
        keyboard_text: None,
    };
    details.compute_keyboard();
    Some(details)
}

fn parse_gitlab_pipeline(
    repo_url: &str, repo_path: &str, author_name: &str, author_url: &str,
    commit_sha: &Option<String>, commit_message: &Option<String>,
    workflow_name: &str, _source: &str,
) -> Option<EventDetails> {
    let branch = env::var("CI_COMMIT_BRANCH").ok();

    let mut details = EventDetails {
        event_type: EventType::WorkflowDispatch,
        author_name: author_name.to_string(),
        author_url: author_url.to_string(),
        repo_name: repo_path.to_string(),
        repo_url: repo_url.to_string(),
        workflow_name: workflow_name.to_string(),
        branch,
        commit_sha: commit_sha.clone(),
        commit_message: commit_message.clone(),
        compare_url: None,
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
    Some(details)
}

fn generic_gitlab_event(
    event_type: EventType,
    repo_url: &str, repo_path: &str, author_name: &str, author_url: &str,
    workflow_name: &str,
) -> EventDetails {
    let mut details = EventDetails {
        event_type,
        author_name: author_name.to_string(),
        author_url: author_url.to_string(),
        repo_name: repo_path.to_string(),
        repo_url: repo_url.to_string(),
        workflow_name: workflow_name.to_string(),
        branch: None,
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
        workflow_inputs: Vec::new(),
        keyboard_url: None,
        keyboard_text: None,
    };
    details.compute_keyboard();
    details
}
