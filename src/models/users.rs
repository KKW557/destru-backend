use serde::{Deserialize, Serialize};
use crate::models::ids::UserID;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub name: Option<String>,
    pub password: Option<String>,
    pub remember: Option<bool>,
}