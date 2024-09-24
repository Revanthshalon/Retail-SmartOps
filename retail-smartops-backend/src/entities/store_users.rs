use uuid::Uuid;

/// Represents the relationship between stores and users in the system.
///
/// This struct is used to map the relationship between stores and their associated users.
pub struct StoreUsers {
    /// The identifier of the store.
    pub store_id: i32,
    /// The identifier of the user associated with the store.
    pub user_id: Uuid,
}
