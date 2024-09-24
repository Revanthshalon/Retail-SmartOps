use crate::repositories::role::RoleRepositoryTrait;
use crate::repositories::user::UserRepositoryTrait;
use sqlx::PgPool;

mod role;
mod user;
mod user_role;

/// Container for all repository instances.
///
/// This struct holds instances of various repositories used in the application.
/// It provides methods to access these repositories.
pub struct RepositoryContainer {
    /// The user repository instance.
    pub user_repo: Box<dyn UserRepositoryTrait>,
    pub role_repo: Box<dyn RoleRepositoryTrait>,
}

impl RepositoryContainer {
    /// Creates a new instance of `RepositoryContainer`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool for the PostgreSQL database.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of `RepositoryContainer`.
    pub fn new(pool: PgPool) -> Self {
        let user_repo = Box::new(user::UserRepository::new(pool.clone()));
        let role_repo = Box::new(role::RoleRepository::new(pool.clone()));
        Self {
            user_repo,
            role_repo,
        }
    }
}
