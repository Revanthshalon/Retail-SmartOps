use serde::{Deserialize, Serialize};

/// Represents a role in the system.
///
/// This struct is used to store role information such as ID and name.
/// It derives `Debug`, `Serialize`, `Deserialize`, and `sqlx::FromRow` for easy
/// debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    /// The unique identifier of the role.
    pub id: i32,
    /// The name of the role.
    pub name: String,
}
