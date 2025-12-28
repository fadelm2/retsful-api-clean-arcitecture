use crate::domain::{
    entity::{address_entity::Address, contact_entity::Contact},
    repository::contact_repository::ContactRepository,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PostgresContactRepository {
    pool: Pool<Postgres>,
}

impl PostgresContactRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContactRepository for PostgresContactRepository {
    async fn create_contact(&self, contact: &Contact) -> Result<Contact, String> {
        let result = sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, user_id, first_name, last_name, email, phone, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING *"
        )
        .bind(contact.id)
        .bind(contact.user_id)
        .bind(&contact.first_name)
        .bind(&contact.last_name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(contact.created_at)
        .bind(contact.updated_at)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update_contact(&self, contact: &Contact) -> Result<Contact, String> {
        let result = sqlx::query_as::<_, Contact>(
            "UPDATE contacts 
             SET first_name = $1, last_name = $2, email = $3, phone = $4, updated_at = $5 
             WHERE id = $6 
             RETURNING *"
        )
        .bind(&contact.first_name)
        .bind(&contact.last_name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(contact.updated_at)
        .bind(contact.id)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn delete_contact(&self, id: &Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM contacts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_contact_by_id(&self, id: &Uuid) -> Result<Option<Contact>, String> {
        let result = sqlx::query_as::<_, Contact>("SELECT * FROM contacts WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_contacts_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Contact>, String> {
        let result = sqlx::query_as::<_, Contact>("SELECT * FROM contacts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(contacts) => Ok(contacts),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn create_address(&self, address: &Address) -> Result<Address, String> {
        let result = sqlx::query_as::<_, Address>(
            "INSERT INTO addresses (id, contact_id, street, city, province, country, postal_code, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
             RETURNING *"
        )
        .bind(address.id)
        .bind(address.contact_id)
        .bind(&address.street)
        .bind(&address.city)
        .bind(&address.province)
        .bind(&address.country)
        .bind(&address.postal_code)
        .bind(address.created_at)
        .bind(address.updated_at)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn update_address(&self, address: &Address) -> Result<Address, String> {
        let result = sqlx::query_as::<_, Address>(
            "UPDATE addresses 
             SET street = $1, city = $2, province = $3, country = $4, postal_code = $5, updated_at = $6
             WHERE id = $7 
             RETURNING *"
        )
        .bind(&address.street)
        .bind(&address.city)
        .bind(&address.province)
        .bind(&address.country)
        .bind(&address.postal_code)
        .bind(address.updated_at)
        .bind(address.id)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        }
    }
    
    async fn delete_address(&self, id: &Uuid) -> Result<(), String> {
         let result = sqlx::query("DELETE FROM addresses WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_address_by_id(&self, id: &Uuid) -> Result<Option<Address>, String> {
        let result = sqlx::query_as::<_, Address>("SELECT * FROM addresses WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(a) => Ok(a),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_addresses_by_contact_id(&self, contact_id: &Uuid) -> Result<Vec<Address>, String> {
        let result = sqlx::query_as::<_, Address>("SELECT * FROM addresses WHERE contact_id = $1")
            .bind(contact_id)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(addresses) => Ok(addresses),
            Err(e) => Err(e.to_string()),
        }
    }
}
