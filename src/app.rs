use crate::delivery::http::handler::user_handler::AppState;
use crate::delivery::http::router::create_router;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::db::postgres::create_pool;
use crate::infrastructure::repository::postgres_contact_repository::PostgresContactRepository;
use crate::infrastructure::repository::postgres_user_repository::PostgresUserRepository;
use crate::usecase::contact_usecase::ContactUsecase;
use crate::usecase::user_usecase::UserUsecase;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use sqlx::{Pool, Postgres};

pub async fn create_app(pool: Pool<Postgres>) -> Router {
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let contact_repo = Arc::new(PostgresContactRepository::new(pool));
    
    let jwt_service = Arc::new(JwtService::new());
    
    let user_usecase = Arc::new(UserUsecase::new(user_repo, jwt_service.clone()));
    let contact_usecase = Arc::new(ContactUsecase::new(contact_repo));

    let app_state = Arc::new(AppState { 
        user_usecase,
        contact_usecase,
        jwt_service,
    });

    create_router(app_state)
}

pub async fn run_app() {
    let pool = create_pool().await;
    let app = create_app(pool).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
