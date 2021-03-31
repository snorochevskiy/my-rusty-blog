use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub text: String,
    pub timestamp: String
}