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
    pub id: Option<u64>,
    pub body: Option<String>,
    #[serde(default)]
    pub state: String,
    pub html_url: String,
}
