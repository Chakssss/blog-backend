use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub name: String,
    pub email: String,
    pub password: String
}