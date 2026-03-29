use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
    pub html_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub login: String,
    pub html_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    #[serde(alias = "content")]
    pub body: Option<String>,
    #[serde(alias = "type", default)]
    pub state: String,
    #[serde(default)]
    pub html_url: String,
}
