use crate::enums::event_enums::GitHubEvent;
use crate::structures::data_structure::DataStructure;
use crate::utils::event_message_utils::{generate_pull_request_message, generate_push_message};
use crate::utils::telegram_utils::send_notify_message;

mod enums;
mod structures;
mod utils;
mod implementations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataStructure::new()?;

    let message = match &data.event {
        GitHubEvent::Push(event) => generate_push_message(&data, event),
        GitHubEvent::PullRequest(event ) => generate_pull_request_message(&data, event),
    };

    send_notify_message(message, &data).await;

    Ok(())
}