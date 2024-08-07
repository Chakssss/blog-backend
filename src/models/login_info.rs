use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}