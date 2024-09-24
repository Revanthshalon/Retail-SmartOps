use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the user hierarchy in the system.
///
/// This struct is used to store information about the reporting structure of users,
/// specifically which user reports to whom. It derives `Debug`, `Serialize`, `Deserialize`,
/// and `sqlx::FromRow` for easy debugging, serialization, deserialization, and database row mapping.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserHierarchy {
    /// The unique identifier of the user.
    pub user_id: Uuid,
    /// The unique identifier of the user to whom this user reports.
    pub reports_to: Uuid,
}
