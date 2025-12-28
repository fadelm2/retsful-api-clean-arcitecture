use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Address {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
