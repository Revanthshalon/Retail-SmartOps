use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data Transfer Object for creating a new user.
///
/// # Fields
///
/// * `username` - The username of the new user.
/// * `email` - The email address of the new user.
/// * `password` - The password for the new user.
#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Data Transfer Object for updating an existing user.
///
/// # Fields
///
/// * `username` - An optional new username for the user.
/// * `email` - An optional new email address for the user.
/// * `password` - An optional new password for the user.
#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

/// Data Transfer Object for responding with user information.
///
/// # Fields
///
/// * `id` - The unique identifier of the user.
/// * `username` - The username of the user.
/// * `email` - The email address of the user.
#[derive(Debug, Serialize)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
