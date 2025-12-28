mod app;
mod delivery;
mod domain;
mod infrastructure;
mod usecase;

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_clean_arcitecture=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    app::run_app().await;
}
