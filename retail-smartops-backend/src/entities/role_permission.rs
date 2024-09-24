use serde::{Deserialize, Serialize};

/// Represents the relationship between roles and permissions in the system.
///
/// This struct is used to map the relationship between roles and their associated permissions.
/// It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::FromRow` for easy
/// debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RolePermission {
    /// The identifier of the role.
    pub role_id: i32,
    /// The identifier of the permission assigned to the role.
    pub permission_id: i32,
}
