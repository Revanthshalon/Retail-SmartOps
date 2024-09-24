use serde::Serialize;
use uuid::Uuid;

/// Data Transfer Object for responding with user role information.
///
/// # Fields
///
/// * `user_id` - The unique identifier of the user.
/// * `role_id` - The identifier of the role assigned to the user.
#[derive(Debug, Serialize)]
pub struct UserRoleResponseDTO {
    pub user_id: Uuid,
    pub role_id: i32,
}
