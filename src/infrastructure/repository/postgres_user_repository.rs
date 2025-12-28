use crate::domain::{entity::user_entity::User, repository::user_repository::UserRepository};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: &User) -> Result<User, String> {
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING *"
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_user_by_id(&self, id: &Uuid) -> Result<Option<User>, String> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }
}
