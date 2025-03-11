use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub name: String,
}

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