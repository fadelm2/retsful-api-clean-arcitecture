use crate::domain::entity::user_entity::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::auth::password::PasswordService;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

pub struct UserUsecase {
    user_repo: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

impl UserUsecase {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repo,
            jwt_service,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<UserResponse, String> {
        req.validate().map_err(|e| e.to_string())?;

        if self.user_repo.find_user_by_email(&req.email).await?.is_some() {
            return Err("Email already exists".to_string());
        }

        let password_hash = PasswordService::hash_password(&req.password)?;

        let new_user = User {
            id: Uuid::new_v4(),
            username: req.username,
            email: req.email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_user = self.user_repo.create_user(&new_user).await?;
        Ok(created_user.into())
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, String> {
        let user = self
            .user_repo
            .find_user_by_email(&req.email)
            .await?
            .ok_or("Invalid credentials")?;

        if !PasswordService::verify_password(&req.password, &user.password_hash)? {
            return Err("Invalid credentials".to_string());
        }

        let token = self.jwt_service.generate_token(user.id)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::user_repository::MockUserRepository;

    #[tokio::test]
    async fn test_register_user_success() {
        let mut mock_repo = MockUserRepository::new();
        let jwt_service = Arc::new(JwtService::new());

        mock_repo
            .expect_find_user_by_email()
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_user()
            .times(1)
            .returning(|u| Ok(u.clone()));

        let usecase = UserUsecase::new(Arc::new(mock_repo), jwt_service);

        let req = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.register(req).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let mut mock_repo = MockUserRepository::new();
        let jwt_service = Arc::new(JwtService::new());

        mock_repo
            .expect_find_user_by_email()
            .times(1)
            .returning(|_| Ok(Some(User {
                id: Uuid::new_v4(),
                username: "existing".to_string(),
                email: "test@example.com".to_string(),
                password_hash: "hash".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })));

        let usecase = UserUsecase::new(Arc::new(mock_repo), jwt_service);

        let req = RegisterRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.register(req).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Email already exists");
    }
}
