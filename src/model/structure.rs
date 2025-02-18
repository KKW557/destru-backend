use crate::model::file::File;
use crate::model::user::UserPreview;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Structure {
    pub id: String,
    pub name: String,
    pub files: Vec<File>,
    pub images: Vec<File>,
    pub summary: String,
    pub description: String,
    pub creators: Vec<UserPreview>,
    pub created: i64,
}

impl Structure {
    pub fn to_preview(&self) -> StructurePreview {
        StructurePreview {
            id: self.id.clone(),
            name: self.name.clone(),
            image: self.images.first().unwrap().url.clone(),
            creator: self.creators.first().unwrap().name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StructurePreview {
    pub id: String,
    pub name: String,
    pub image: String,
    pub creator: String,
}

impl StructurePreview {
    pub fn from_structure(structure: Structure) -> StructurePreview {
        StructurePreview {
            id: structure.id.clone(),
            name: structure.name.clone(),
            image: structure.images.first().unwrap().url.clone(),
            creator: structure.creators.first().unwrap().name.clone(),
        }
    }
}