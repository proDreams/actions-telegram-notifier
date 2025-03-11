use crate::structures::event_type_structures::base_structures::{Repository, Sender};
use crate::structures::event_type_structures::push_structures::HeadCommit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PushEvent {
    pub compare: String,
    pub head_commit: HeadCommit,
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {}
