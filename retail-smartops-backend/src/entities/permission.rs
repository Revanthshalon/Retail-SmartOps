use serde::{Deserialize, Serialize};

/// Represents a permission in the system.
///
/// This struct is used to store permission information such as ID, name, and various
/// boolean flags indicating the types of actions that can be performed (read, write, delete, update).
/// It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::FromRow` for easy
/// debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Permission {
    /// The unique identifier of the permission.
    pub id: i32,
    /// The entity_name of the permission.
    pub entity_name: String,
    /// Indicates if the permission allows reading.
    pub can_read: bool,
    /// Indicates if the permission allows writing.
    pub can_write: bool,
    /// Indicates if the permission allows deleting.
    pub can_delete: bool,
    /// Indicates if the permission allows updating.
    pub can_update: bool,
}
