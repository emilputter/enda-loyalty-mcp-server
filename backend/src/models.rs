use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,  Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}