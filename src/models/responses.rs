use serde::Serialize;
use crate::models::ids::UserID;
use crate::models::pagination::Pagination;
use crate::models::structures::{Structure, StructurePreview};
use crate::models::users::User;

#[derive(Serialize)]
pub struct StructureResponse {
    pub structure: Structure,
}

#[derive(Serialize)]
pub struct StructuresResponse {
    pub structures: Vec<StructurePreview>,
    pub pagination: Pagination,
}

#[derive(Serialize)]
pub enum UserError {
    InvalidName,
    InvalidPassword,
    NameExists,
}

#[derive(Serialize)]
pub struct RegisterErrorResponse {
    pub reason: UserError,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: UserID,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Serialize)]
pub struct UserNameResponse {
    pub name: String,
}