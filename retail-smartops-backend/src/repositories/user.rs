use crate::errors::AppError;
use crate::models::user::{CreateUserDTO, UpdateUserDTO, UserResponseDTO};
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for user-related database operations.
pub struct UserRepository {
    /// Connection pool for the PostgreSQL database.
    pool: PgPool,
}

impl UserRepository {
    /// Creates a new instance of `UserRepository`.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool for the PostgreSQL database.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Checks if a username already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - `Ok(true)` if the username exists, `Ok(false)` otherwise, or an `AppError`.
    async fn check_if_username_exists(&self, username: &str) -> Result<bool, AppError> {
        let username_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        let count = match username_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if username exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }

    /// Checks if an email already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `email` - The email to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - `Ok(true)` if the email exists, `Ok(false)` otherwise, or an `AppError`.
    async fn check_if_email_exists(&self, email: &str) -> Result<bool, AppError> {
        let email_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        let count = match email_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if email exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }

    /// Checks if a user ID already exists in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, AppError>` - `Ok(true)` if the ID exists, `Ok(false)` otherwise, or an `AppError`.
    async fn check_if_id_exists(&self, id: &Uuid) -> Result<bool, AppError> {
        let id_count = sqlx::query!("SELECT COUNT(id) as count FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?
            .count;

        let count = match id_count {
            Some(count) => count,
            None => {
                return Err(AppError::InternalServerError(
                    "Failed to check if id exists".to_string(),
                ))
            }
        };

        Ok(count > 0)
    }
}

/// Trait defining the user repository operations.
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    /// Creates a new user in the database.
    ///
    /// # Arguments
    ///
    /// * `payload` - The data transfer object containing user creation details.
    ///
    /// # Returns
    ///
    /// * `Result<UserResponseDTO, AppError>` - The created user or an `AppError`.
    async fn create_user(&self, payload: CreateUserDTO) -> Result<UserResponseDTO, AppError>;

    /// Retrieves a user by their ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID.
    ///
    /// # Returns
    ///
    /// * `Result<UserResponseDTO, AppError>` - The user details or an `AppError`.
    async fn get_user_by_id(&self, id: Uuid) -> Result<UserResponseDTO, AppError>;

    /// Updates an existing user in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID.
    /// * `payload` - The data transfer object containing user update details.
    ///
    /// # Returns
    ///
    /// * `Result<UserResponseDTO, AppError>` - The updated user or an `AppError`.
    async fn update_user(
        &self,
        id: Uuid,
        payload: UpdateUserDTO,
    ) -> Result<UserResponseDTO, AppError>;

    /// Deletes a user from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID.
    ///
    /// # Returns
    ///
    /// * `Result<(), AppError>` - `Ok(())` if the user was deleted, or an `AppError`.
    async fn delete_user(&self, id: Uuid) -> Result<(), AppError>;

    /// Retrieves all users from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<UserResponseDTO>, AppError>` - A list of users or an `AppError`.
    async fn get_all_users(&self) -> Result<Vec<UserResponseDTO>, AppError>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, payload: CreateUserDTO) -> Result<UserResponseDTO, AppError> {
        if self.check_if_email_exists(&payload.email).await?
            || self.check_if_username_exists(&payload.username).await?
        {
            return Err(AppError::Conflict);
        }

        let user = sqlx::query_as!(
            UserResponseDTO,
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email
            "#,
            payload.username,
            payload.email,
            payload.password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserResponseDTO, AppError> {
        let user_optional = sqlx::query_as!(
            UserResponseDTO,
            r#"
            SELECT id, username, email
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match user_optional {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound),
        }
    }

    async fn update_user(
        &self,
        id: Uuid,
        payload: UpdateUserDTO,
    ) -> Result<UserResponseDTO, AppError> {
        if !self.check_if_id_exists(&id).await? {
            return Err(AppError::NotFound);
        }

        let updated_user = sqlx::query_as!(
            UserResponseDTO,
            r#"
            UPDATE users
            SET username = COALESCE($2, username),
                email = COALESCE($3, email),
                password = COALESCE($4, password)
            WHERE id = $1
            RETURNING id, username, email
            "#,
            id,
            payload.username,
            payload.email,
            payload.password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        if !self.check_if_id_exists(&id).await? {
            return Err(AppError::NotFound);
        }

        let query_result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if query_result.rows_affected() == 0 {
            return Err(AppError::InternalServerError(
                "Failed to delete user".to_string(),
            ));
        }

        Ok(())
    }

    async fn get_all_users(&self) -> Result<Vec<UserResponseDTO>, AppError> {
        let users = sqlx::query_as!(
            UserResponseDTO,
            r#"
            SELECT id, username, email
            FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
