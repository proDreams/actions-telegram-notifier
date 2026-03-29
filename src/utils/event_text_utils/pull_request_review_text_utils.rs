use crate::structures::event_type_structures::base_structures::Review;

pub fn get_pull_request_review_input_title(title: &str, state: &str) -> String {
    let (icon, default_text) = match state.to_lowercase().as_str() {
        "approved" | "pull_request_review_approved" => ("✅", "Pull Request Approved"),
        // В Gitea отказ обычно приходит как rejected, добавляем на всякий случай
        "changes_requested" | "pull_request_review_rejected" => ("❌", "Changes Requested"),
        "commented" | "pull_request_review_comment" => ("💬", "Review Comment Added"),
        "dismissed" | "pull_request_review_dismissed" => ("🗑️", "Review Dismissed"),
        _ => ("🔍", "Pull Request Reviewed"),
    };

    if title.is_empty() {
        format!("{} <b>{}</b>\n", icon, default_text)
    } else {
        format!("{} {}\n", icon, title)
    }
}

pub fn get_review_details(review: &Review, review_url: &String) -> String {
    let mut details = format!(
        "🔗 <b>Review Link:</b> <a href='{}'>Click here</a>\n",
        review_url
    );

    if let Some(body) = &review.body {
        if !body.trim().is_empty() {
            let truncated_body = if body.chars().count() > 150 {
                format!("{}...", body.chars().take(150).collect::<String>())
            } else {
                body.to_string()
            };
            details.push_str(&format!(
                "📝 <b>Comment:</b>\n<blockquote>{}</blockquote>\n",
                truncated_body
            ));
        }
    }

    details
}
