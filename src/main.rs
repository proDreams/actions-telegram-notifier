use crate::structures::data_structure::DataStructure;
use crate::utils::event_message_utils::generate_current_message;
use crate::utils::telegram_utils::send_notify_message;

mod enums;
mod implementations;
mod serializers;
mod structures;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = DataStructure::new()?;

    let message = generate_current_message(&data);

    send_notify_message(message, &data).await;

    Ok(())
}
