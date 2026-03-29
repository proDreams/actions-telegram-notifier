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
    pub id: u64,
    pub user: Sender,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
}
