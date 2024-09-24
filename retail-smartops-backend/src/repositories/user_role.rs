use crate::errors::AppError;
use crate::models::user_role::UserRoleResponseDTO;
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for managing user roles in the database.
pub struct UserRoleRepository {
    pool: PgPool,
}

impl UserRoleRepository {
    /// Creates a new instance of `UserRoleRepository`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to the PostgreSQL database.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Checks if a user role exists in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - Returns `Ok(true)` if the user role exists, `Ok(false)` otherwise.
    ///   Returns an `AppError` if an error occurs.
    async fn check_if_user_role_exists(
        &self,
        user_id: &Uuid,
        role_id: i32,
    ) -> Result<bool, AppError> {
        let user_role_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_roles
            WHERE user_id = $1 AND role_id = $2
            "#,
            user_id,
            role_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        let count = match user_role_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if user role exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }
}

/// Trait defining the operations for managing user roles.
#[async_trait]
pub trait UserRoleRepositoryTrait: Send + Sync {
    /// Adds a user role to the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<UserRoleResponseDTO, AppError>` - Returns the added user role or an `AppError` if an error occurs.
    async fn add_user_role(
        &self,
        user_id: uuid::Uuid,
        role_id: i32,
    ) -> Result<UserRoleResponseDTO, AppError>;

    /// Updates a user role in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<UserRoleResponseDTO, AppError>` - Returns the updated user role or an `AppError` if an error occurs.
    async fn update_user_role(
        &self,
        user_id: uuid::Uuid,
        role_id: i32,
    ) -> Result<UserRoleResponseDTO, AppError>;

    /// Deletes a user role from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<(), AppError>` - Returns `Ok(())` if the user role was deleted or an `AppError` if an error occurs.
    async fn delete_user_role(&self, user_id: uuid::Uuid, role_id: i32) -> Result<(), AppError>;
}

#[async_trait]
impl UserRoleRepositoryTrait for UserRoleRepository {
    /// Adds a user role to the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<UserRoleResponseDTO, AppError>` - Returns the added user role or an `AppError` if an error occurs.
    async fn add_user_role(
        &self,
        user_id: Uuid,
        role_id: i32,
    ) -> Result<UserRoleResponseDTO, AppError> {
        if self.check_if_user_role_exists(&user_id, role_id).await? {
            return Err(AppError::Conflict);
        }

        let user_role = sqlx::query_as!(
            UserRoleResponseDTO,
            r#"
            INSERT INTO user_roles (user_id, role_id)
            VALUES ($1, $2)
            RETURNING user_id, role_id
            "#,
            user_id,
            role_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_role)
    }

    /// Updates a user role in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<UserRoleResponseDTO, AppError>` - Returns the updated user role or an `AppError` if an error occurs.
    async fn update_user_role(
        &self,
        user_id: Uuid,
        role_id: i32,
    ) -> Result<UserRoleResponseDTO, AppError> {
        if !self.check_if_user_role_exists(&user_id, role_id).await? {
            return Err(AppError::NotFound);
        }

        let user_role = sqlx::query_as!(
            UserRoleResponseDTO,
            r#"
            UPDATE user_roles
            SET role_id = $1
            WHERE user_id = $2
            RETURNING user_id, role_id
            "#,
            role_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_role)
    }

    /// Deletes a user role from the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user.
    /// * `role_id` - The ID of the role.
    ///
    /// # Returns
    ///
    /// * `Result<(), AppError>` - Returns `Ok(())` if the user role was deleted or an `AppError` if an error occurs.
    async fn delete_user_role(&self, user_id: Uuid, role_id: i32) -> Result<(), AppError> {
        if !self.check_if_user_role_exists(&user_id, role_id).await? {
            return Err(AppError::NotFound);
        }

        let query_result = sqlx::query!(
            r#"
            DELETE FROM user_roles
            WHERE user_id = $1 AND role_id = $2
            "#,
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await?;

        if query_result.rows_affected() == 0 {
            return Err(AppError::InternalServerError(
                "Failed to delete user role".to_string(),
            ));
        }

        Ok(())
    }
}
