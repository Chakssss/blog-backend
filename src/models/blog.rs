use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub title: String,
    pub description: String,
    pub content: String,
    pub author: String
}

// blog model