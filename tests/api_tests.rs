use rust_clean_arcitecture::app::create_app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for collecting body
use tower::ServiceExt;
use sqlx::PgPool;
use serde_json::{json, Value};

#[sqlx::test]
async fn test_register_user(pool: PgPool) {
    let app = create_app(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["username"], "testuser");
    assert_eq!(body["email"], "test@example.com");
}

#[sqlx::test]
async fn test_login_user(pool: PgPool) {
    let app = create_app(pool).await;

    // Register first
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/register")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Login
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@example.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert!(body["token"].is_string());
}

#[sqlx::test]
async fn test_contact_flow(pool: PgPool) {
    let app = create_app(pool).await;

    // 1. Register
    app.clone()
        .oneshot(
             Request::builder()
                .method("POST")
                .uri("/users/register")
                .header("content-type", "application/json")
                .body(Body::from(json!({"username": "u", "email": "u@e.com", "password": "p"}).to_string())).unwrap()
        ).await.unwrap();

    // 2. Login to get token
    let login_res = app.clone().oneshot(
             Request::builder()
                .method("POST")
                .uri("/users/login")
                .header("content-type", "application/json")
                .body(Body::from(json!({"email": "u@e.com", "password": "p"}).to_string())).unwrap()
        ).await.unwrap();
    
    let body = login_res.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    let token = body["token"].as_str().unwrap();
    let auth_header = format!("Bearer {}", token);

    // 3. Create Contact
    let create_res = app.clone().oneshot(
            Request::builder()
            .method("POST")
            .uri("/contacts")
            .header("content-type", "application/json")
            .header("Authorization", &auth_header)
            .body(Body::from(json!({
                "first_name": "Contact",
                "email": "c@example.com", 
                "phone": "123"
            }).to_string())).unwrap()
        ).await.unwrap();
    
    assert_eq!(create_res.status(), StatusCode::CREATED);
    let c_body = create_res.into_body().collect().await.unwrap().to_bytes();
    let c_json: Value = serde_json::from_slice(&c_body).unwrap();
    let contact_id = c_json["id"].as_str().unwrap();

    // 4. Create Address
    let addr_res = app.clone().oneshot(
            Request::builder()
            .method("POST")
            .uri(format!("/contacts/{}/addresses", contact_id))
            .header("content-type", "application/json")
            .header("Authorization", &auth_header)
            .body(Body::from(json!({
                "country": "Indonesia",
                "city": "Jakarta"
            }).to_string())).unwrap()
        ).await.unwrap();
    
    assert_eq!(addr_res.status(), StatusCode::CREATED);

    // 5. Get Contact (verify address is included if logic implementation supports it, 
    // or just verify contact retrieval works)
    let get_res = app.clone().oneshot(
            Request::builder()
            .method("GET")
            .uri(format!("/contacts/{}", contact_id))
            .header("Authorization", &auth_header)
            .body(Body::empty()).unwrap()
        ).await.unwrap();

   assert_eq!(get_res.status(), StatusCode::OK);
   let get_body = get_res.into_body().collect().await.unwrap().to_bytes();
   let get_json: Value = serde_json::from_slice(&get_body).unwrap();
   assert_eq!(get_json["first_name"], "Contact");
   // Check if addresses are returned (based on our usecase, they should be)
   assert!(get_json["addresses"].is_array());
   assert_eq!(get_json["addresses"].as_array().unwrap().len(), 1);
}
