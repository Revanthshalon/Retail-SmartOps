use serde::{Deserialize, Serialize};

/// Data Transfer Object for creating a new role.
///
/// This struct is used to deserialize the payload when creating a new role.
///
/// # Fields
///
/// * `name` - The name of the role.
#[derive(Debug, Deserialize)]
pub struct CreateRoleDTO {
    pub name: String,
}

/// Data Transfer Object for updating an existing role.
///
/// This struct is used to deserialize the payload when updating an existing role.
///
/// # Fields
///
/// * `name` - The new name of the role. This field is optional.
#[derive(Debug, Deserialize)]
pub struct UpdateRoleDTO {
    pub name: Option<String>,
}

/// Data Transfer Object for responding with role details.
///
/// This struct is used to serialize the role details when responding to a client.
///
/// # Fields
///
/// * `id` - The unique identifier of the role.
/// * `name` - The name of the role.
#[derive(Debug, Serialize)]
pub struct RoleResponseDTO {
    pub id: i32,
    pub name: String,
}
