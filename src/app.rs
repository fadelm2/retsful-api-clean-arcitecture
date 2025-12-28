use crate::delivery::http::handler::user_handler::AppState;
use crate::delivery::http::router::create_router;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::db::postgres::create_pool;
use crate::infrastructure::repository::postgres_user_repository::PostgresUserRepository;
use crate::usecase::user_usecase::UserUsecase;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run_app() {
    let pool = create_pool().await;
    let user_repo = Arc::new(PostgresUserRepository::new(pool));
    let jwt_service = Arc::new(JwtService::new());
    let user_usecase = Arc::new(UserUsecase::new(user_repo, jwt_service));

    let app_state = Arc::new(AppState { user_usecase });

    let app = create_router(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
