use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::PushEvent;
use crate::utils::text_utils::{
    escape_markdown_v2, generate_input_message, generate_notify_fields, get_input_title,
};

pub fn generate_push_message(data: &DataStructure, event: &PushEvent) -> String {
    let mut message = "".to_owned();

    message += &*get_input_title(data.title.as_deref().unwrap_or_default(), &data.status);

    message += &*generate_notify_fields(data, &event);

    message += &*generate_input_message(data.message.as_deref().unwrap_or_default());

    message += &*generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}
