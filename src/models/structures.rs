use crate::models::files::File;
use crate::models::ids::StructureID;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::models::users::UserPreview;

#[derive(Serialize)]
pub struct Structure {
    pub id: StructureID,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub files: Vec<File>,
    pub images: Vec<File>,
    pub creators: Vec<UserPreview>,
    pub created: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct DbStructure {
    pub id: StructureID,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub created: NaiveDateTime,
}

#[derive(Serialize)]
pub struct StructurePreview {
    pub id: StructureID,
    pub name: String,
    pub image: Option<String>,
    pub creator: Option<String>,
}