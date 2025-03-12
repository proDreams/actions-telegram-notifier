use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::{PullRequestEvent, PushEvent};
use crate::utils::text_utils::{
    generate_input_message, generate_pull_request_notify_fields, generate_push_notify_fields,
    get_pull_request_input_title, get_pull_request_title, get_push_input_title,
};

pub fn generate_push_message(data: &DataStructure, event: &PushEvent) -> String {
    let mut message: String = "".to_owned();

    message += &*get_push_input_title(data.title.as_deref().unwrap_or_default(), &data.status);

    message += &*generate_push_notify_fields(data, &event);

    message += &*generate_input_message(data.message.as_deref().unwrap_or_default());

    message += &*generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

pub fn generate_pull_request_message(data: &DataStructure, event: &PullRequestEvent) -> String {
    let mut message = "".to_owned();

    message += &*get_pull_request_input_title(
        data.title.as_deref().unwrap_or_default(),
        &event.action,
        &event.number,
    );
    
    message += &*get_pull_request_title(&event.pull_request);

    message += &*generate_pull_request_notify_fields(data, &event);

    message += &*generate_input_message(data.message.as_deref().unwrap_or_default());

    message += &*generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}
