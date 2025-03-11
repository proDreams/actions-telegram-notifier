use crate::structures::event_type_structures::base_structures::{Repository, Sender};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct PushEvent {
    pub compare: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub sender: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {}
