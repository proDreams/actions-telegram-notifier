use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct  PullRequestData {
    pub html_url: String,
    pub title: String,
    pub merged: bool,
}