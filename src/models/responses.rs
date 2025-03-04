use serde::Serialize;
use crate::models::structures::{Structure, StructurePreview};

// #[derive(Serialize)]
// pub struct ErrorResponse {
//     pub success: bool,
//     pub cause: String,
// }
//
// impl ErrorResponse {
//     pub fn new(cause: String) -> Self {
//         ErrorResponse {
//             success: false,
//             cause,
//         }
//     }
// }

#[derive(Serialize)]
pub struct NotFoundResponse {
    pub success: bool,
}

impl NotFoundResponse {
    pub fn new() -> Self {
        NotFoundResponse {
            success: false,
        }
    }
}

#[derive(Serialize)]
pub struct StructureResponse {
    pub success: bool,
    pub structure: Structure,
}

impl StructureResponse {
    pub fn new(structure: Structure) -> Self {
        StructureResponse {
            success: true,
            structure
        }
    }
}

#[derive(Serialize)]
pub struct StructuresResponse {
    pub success: bool,
    pub structures: Vec<StructurePreview>,
}

impl StructuresResponse {
    pub fn new(structures: Vec<StructurePreview>) -> Self {
        StructuresResponse {
            success: true,
            structures,
        }
    }
}