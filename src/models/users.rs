use serde::{Deserialize, Serialize};
use crate::models::ids::UserID;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
    pub name: String,
    pub avatar: String,
}