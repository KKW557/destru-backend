use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserPreview {
    pub id: String,
    pub name: String,
    pub avatar: String
}