use crate::structures::data_structure::DataStructure;
use crate::structures::event_structures::{PullRequestEvent, PushEvent, WorkflowDispatchEvent};
use crate::utils::event_text_utils::pull_request_text_utils::{
    get_pull_request_input_title, get_pull_request_title,
};
use crate::utils::event_text_utils::push_text_utils::{
    generate_push_notify_fields, get_push_input_title,
};
use crate::utils::event_text_utils::workflow_dispatch_text_utils::{
    generate_workflow_dispatch_inputs_message, generate_workflow_dispatch_notify_fields,
    get_workflow_dispatch_input_title, get_workflow_dispatch_status,
};
use crate::utils::text_utils::{generate_general_fields, generate_input_message};

pub fn generate_push_message(data: &DataStructure, event: &PushEvent) -> String {
    let mut message: String = "".to_owned();

    message += &*get_push_input_title(data.title.as_deref().unwrap_or_default(), &data.status);

    message += &*generate_general_fields(
        &data.notify_fields,
        &event.sender.html_url,
        &event.sender.login,
        &event.repository.html_url,
        &event.repository.full_name,
        &data.workflow,
    );

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
        &event.pull_request.merged,
    );

    message += &*get_pull_request_title(&event.pull_request);

    message += &*generate_general_fields(
        &data.notify_fields,
        &event.sender.html_url,
        &event.sender.login,
        &event.repository.html_url,
        &event.repository.full_name,
        &data.workflow,
    );

    message += &*generate_input_message(data.message.as_deref().unwrap_or_default());

    message += &*generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}

pub fn generate_workflow_dispatch_message(
    data: &DataStructure,
    event: &WorkflowDispatchEvent,
) -> String {
    let mut message = "".to_owned();

    message += &*get_workflow_dispatch_input_title(data.title.as_deref().unwrap_or_default());
    
    message += &*get_workflow_dispatch_status(&data.status);

    message += &*generate_general_fields(
        &data.notify_fields,
        &event.sender.html_url,
        &event.sender.login,
        &event.repository.html_url,
        &event.repository.full_name,
        &data.workflow,
    );

    message += &*generate_workflow_dispatch_notify_fields(data, event);

    message += &*generate_workflow_dispatch_inputs_message(&event.inputs);
    
    message += &*generate_input_message(data.message.as_deref().unwrap_or_default());

    message += &*generate_input_message(data.footer.as_deref().unwrap_or_default());

    message
}
