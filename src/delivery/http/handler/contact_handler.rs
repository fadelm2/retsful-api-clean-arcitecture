use crate::delivery::http::handler::user_handler::AppState;
use crate::infrastructure::auth::jwt::Claims;
use crate::usecase::contact_usecase::{CreateAddressRequest, CreateContactRequest, UpdateContactRequest};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

// Helper to extract user_id from JWT in headers
fn extract_user_id(
    headers: &HeaderMap,
    app_state: &Arc<AppState>,
) -> Result<Uuid, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()));
    }

    let token = &auth_header[7..];
    let claims = app_state
        .jwt_service // We need to expose jwt_service in AppState or similar.
        // Actually AppState currently only has user_usecase. We need to add jwt_service there or use middleware.
        // For simplicity, let's assume we can access jwt_service via AppState or similar mechanism.
        // Wait, AppState definition in user_handler.rs only has user_usecase.
        // I need to update AppState first to include jwt_service or contact_usecase.
        .verify_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    Uuid::parse_str(&claims.sub).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string()))
}

// Handler functions

pub async fn create_contact(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateContactRequest>,
) -> impl IntoResponse {
    let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.create_contact(user_id, payload).await {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

pub async fn update_contact(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(contact_id): Path<Uuid>,
    Json(payload): Json<UpdateContactRequest>,
) -> impl IntoResponse {
    let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.update_contact(user_id, contact_id, payload).await {
        Ok(contact) => (StatusCode::OK, Json(contact)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

pub async fn search_contacts(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.search_contacts(user_id).await {
        Ok(contacts) => (StatusCode::OK, Json(contacts)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn get_contact(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(contact_id): Path<Uuid>,
) -> impl IntoResponse {
     let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.get_contact(user_id, contact_id).await {
        Ok(contact) => (StatusCode::OK, Json(contact)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e).into_response(),
    }
}

pub async fn delete_contact(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(contact_id): Path<Uuid>,
) -> impl IntoResponse {
     let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.delete_contact(user_id, contact_id).await {
        Ok(_) => (StatusCode::OK, "Contact deleted").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

pub async fn create_address(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(contact_id): Path<Uuid>,
    Json(payload): Json<CreateAddressRequest>,
) -> impl IntoResponse {
     let user_id = match extract_user_id(&headers, &state) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    match state.contact_usecase.create_address(user_id, contact_id, payload).await {
        Ok(address) => (StatusCode::CREATED, Json(address)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
