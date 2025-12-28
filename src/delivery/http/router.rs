use crate::delivery::http::handler::contact_handler::{
    create_address, create_contact, delete_contact, get_contact, search_contacts, update_contact,
};
use crate::delivery::http::handler::user_handler::{login, register, AppState};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .route("/contacts", post(create_contact).get(search_contacts))
        .route(
            "/contacts/:contact_id",
            get(get_contact).put(update_contact).delete(delete_contact),
        )
        .route("/contacts/:contact_id/addresses", post(create_address))
        .with_state(app_state)
}
