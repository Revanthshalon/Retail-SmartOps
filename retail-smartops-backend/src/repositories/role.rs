use crate::errors::AppError;
use crate::models::role::{CreateRoleDTO, RoleResponseDTO, UpdateRoleDTO};
use axum::async_trait;
use sqlx::PgPool;

/// Repository for role-related database operations.
pub struct RoleRepository {
    /// Connection pool for the PostgreSQL database.
    pool: PgPool,
}

impl RoleRepository {
    /// Creates a new instance of `RoleRepository`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool for the PostgreSQL database.
    pub fn new(pool: PgPool) -> Self {
        RoleRepository { pool }
    }

    /// Checks if a role with the given name already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the role to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - `Ok(true)` if the role exists, `Ok(false)` otherwise, or an `AppError`.
    pub async fn check_if_role_exists(&self, name: &str) -> Result<bool, AppError> {
        let role_count = sqlx::query!("SELECT COUNT(*) as count FROM roles WHERE name = $1", name)
            .fetch_one(&self.pool)
            .await?
            .count;

        let count = match role_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if role exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }

    /// Checks if a role with the given ID already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the role to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - `Ok(true)` if the role exists, `Ok(false)` otherwise, or an `AppError`.
    pub async fn check_if_id_exists(&self, id: i32) -> Result<bool, AppError> {
        let role_count = sqlx::query!("SELECT COUNT(*) as count FROM roles WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?
            .count;

        let count = match role_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if role exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }
}

/// Trait defining the role repository operations.
#[async_trait]
pub trait RoleRepositoryTrait: Send + Sync {
    /// Creates a new role in the database.
    ///
    /// # Arguments
    ///
    /// * `payload` - The data transfer object containing role creation details.
    ///
    /// # Returns
    ///
    /// * `Result<RoleResponseDTO, AppError>` - The created role or an `AppError`.
    async fn create_role(&self, payload: CreateRoleDTO) -> Result<RoleResponseDTO, AppError>;

    /// Retrieves a role by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The role ID.
    ///
    /// # Returns
    ///
    /// * `Result<RoleResponseDTO, AppError>` - The role details or an `AppError`.
    async fn get_role_by_id(&self, id: i32) -> Result<RoleResponseDTO, AppError>;

    /// Updates an existing role in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The role ID.
    /// * `payload` - The data transfer object containing role update details.
    ///
    /// # Returns
    ///
    /// * `Result<RoleResponseDTO, AppError>` - The updated role or an `AppError`.
    async fn update_role(
        &self,
        id: i32,
        payload: UpdateRoleDTO,
    ) -> Result<RoleResponseDTO, AppError>;

    /// Deletes a role from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The role ID.
    ///
    /// # Returns
    ///
    /// * `Result<(), AppError>` - `Ok(())` if the role was deleted, or an `AppError`.
    async fn delete_role(&self, id: i32) -> Result<(), AppError>;

    /// Retrieves all roles from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<RoleResponseDTO>, AppError>` - A list of roles or an `AppError`.
    async fn get_roles(&self) -> Result<Vec<RoleResponseDTO>, AppError>;
}

#[async_trait]
impl RoleRepositoryTrait for RoleRepository {
    async fn create_role(&self, payload: CreateRoleDTO) -> Result<RoleResponseDTO, AppError> {
        if self.check_if_role_exists(&payload.name).await? {
            return Err(AppError::Conflict);
        }

        let role = sqlx::query_as!(
            RoleResponseDTO,
            r#"INSERT INTO roles (name) VALUES ($1) RETURNING id, name"#,
            payload.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(role)
    }

    async fn get_role_by_id(&self, id: i32) -> Result<RoleResponseDTO, AppError> {
        let role_option = sqlx::query_as!(
            RoleResponseDTO,
            r#"SELECT id, name FROM roles WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match role_option {
            Some(role) => Ok(role),
            None => Err(AppError::NotFound),
        }
    }

    async fn update_role(
        &self,
        id: i32,
        payload: UpdateRoleDTO,
    ) -> Result<RoleResponseDTO, AppError> {
        if !self.check_if_id_exists(id).await? {
            return Err(AppError::NotFound);
        }

        let role = sqlx::query_as!(
            RoleResponseDTO,
            r#"UPDATE roles SET name = COALESCE($1, name) WHERE id = $2 RETURNING id, name"#,
            payload.name,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(role)
    }

    async fn delete_role(&self, id: i32) -> Result<(), AppError> {
        if !self.check_if_id_exists(id).await? {
            return Err(AppError::NotFound);
        }

        let query_result = sqlx::query!("DELETE FROM roles WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if query_result.rows_affected() == 0 {
            return Err(AppError::InternalServerError(
                "Failed to delete role".to_string(),
            ));
        }

        Ok(())
    }

    async fn get_roles(&self) -> Result<Vec<RoleResponseDTO>, AppError> {
        let roles = sqlx::query_as!(RoleResponseDTO, r#"SELECT id, name FROM roles"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(roles)
    }
}
