use crate::serializers::workflow_dispatch_input_keys::deserialize_input_pairs;
use crate::enums::workflow_enums::PullRequestAction;
use crate::structures::event_type_structures::base_structures::{Repository, Sender};
use crate::structures::event_type_structures::pull_request_structures::PullRequestData;
use crate::structures::event_type_structures::push_structures::HeadCommit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PushEvent {
    pub compare: Option<String>,
    pub compare_url: Option<String>,
    pub head_commit: HeadCommit,
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub pull_request: PullRequestData,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowDispatchEvent {
    #[serde(default, deserialize_with = "deserialize_input_pairs")]
    pub inputs: Vec<(String, String)>,
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub sender: Sender,
    pub workflow: String,
}
