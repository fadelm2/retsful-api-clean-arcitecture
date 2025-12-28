use crate::domain::{
    entity::{address_entity::Address, contact_entity::Contact},
    repository::contact_repository::ContactRepository,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateContactRequest {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    pub last_name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(min = 3, message = "Phone must be at least 3 characters"))]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateContactRequest {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAddressRequest {
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    #[validate(length(min = 1, message = "Country is required"))]
    pub country: String,
    pub postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateAddressRequest {
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub addresses: Vec<AddressResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub id: Uuid,
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
}

impl From<Contact> for ContactResponse {
    fn from(c: Contact) -> Self {
        Self {
            id: c.id,
            first_name: c.first_name,
            last_name: c.last_name,
            email: c.email,
            phone: c.phone,
            addresses: vec![], // Populated separately if needed
        }
    }
}

impl From<Address> for AddressResponse {
    fn from(a: Address) -> Self {
        Self {
            id: a.id,
            street: a.street,
            city: a.city,
            province: a.province,
            country: a.country,
            postal_code: a.postal_code,
        }
    }
}

pub struct ContactUsecase {
    repo: Arc<dyn ContactRepository>,
}

impl ContactUsecase {
    pub fn new(repo: Arc<dyn ContactRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_contact(
        &self,
        user_id: Uuid,
        req: CreateContactRequest,
    ) -> Result<ContactResponse, String> {
        req.validate().map_err(|e| e.to_string())?;

        let new_contact = Contact {
            id: Uuid::new_v4(),
            user_id,
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            phone: req.phone,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_contact = self.repo.create_contact(&new_contact).await?;
        Ok(created_contact.into())
    }

    pub async fn update_contact(
        &self,
        user_id: Uuid,
        contact_id: Uuid,
        req: UpdateContactRequest,
    ) -> Result<ContactResponse, String> {
        req.validate().map_err(|e| e.to_string())?;

        let mut contact = self
            .repo
            .find_contact_by_id(&contact_id)
            .await?
            .ok_or("Contact not found")?;

        if contact.user_id != user_id {
            return Err("Unauthorized".to_string());
        }

        if let Some(first_name) = req.first_name {
            contact.first_name = first_name;
        }
        if let Some(last_name) = req.last_name {
            contact.last_name = Some(last_name);
        }
        if let Some(email) = req.email {
            contact.email = Some(email);
        }
        if let Some(phone) = req.phone {
            contact.phone = Some(phone);
        }

        contact.updated_at = Utc::now();

        let updated_contact = self.repo.update_contact(&contact).await?;
        Ok(updated_contact.into())
    }

    pub async fn delete_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<(), String> {
        let contact = self
            .repo
            .find_contact_by_id(&contact_id)
            .await?
            .ok_or("Contact not found")?;

        if contact.user_id != user_id {
            return Err("Unauthorized".to_string());
        }

        self.repo.delete_contact(&contact_id).await
    }

    pub async fn search_contacts(&self, user_id: Uuid) -> Result<Vec<ContactResponse>, String> {
        let contacts = self.repo.find_contacts_by_user_id(&user_id).await?;
        
        // For each contact, we might want to fetch addresses.
        // Doing this in a loop resembles N+1 query problem, but allowed for simplicity here.
        // In optimized real world, we would use a join query in repository.
        
        let mut responses = Vec::new();
        for contact in contacts {
            let addresses = self.repo.find_addresses_by_contact_id(&contact.id).await?;
            let mut response: ContactResponse = contact.into();
            response.addresses = addresses.into_iter().map(Into::into).collect();
            responses.push(response);
        }

        Ok(responses)
    }

    pub async fn get_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<ContactResponse, String> {
        let contact = self
            .repo
            .find_contact_by_id(&contact_id)
            .await?
            .ok_or("Contact not found")?;

        if contact.user_id != user_id {
            return Err("Unauthorized".to_string());
        }

        let addresses = self.repo.find_addresses_by_contact_id(&contact.id).await?;
        let mut response: ContactResponse = contact.into();
        response.addresses = addresses.into_iter().map(Into::into).collect();

        Ok(response)
    }

    pub async fn create_address(
        &self,
        user_id: Uuid,
        contact_id: Uuid,
        req: CreateAddressRequest,
    ) -> Result<AddressResponse, String> {
        req.validate().map_err(|e| e.to_string())?;

        let contact = self
            .repo
            .find_contact_by_id(&contact_id)
            .await?
            .ok_or("Contact not found")?;

        // Verify ownership
        if contact.user_id != user_id {
            return Err("Unauthorized".to_string());
        }

        let new_address = Address {
            id: Uuid::new_v4(),
            contact_id,
            street: req.street,
            city: req.city,
            province: req.province,
            country: req.country,
            postal_code: req.postal_code,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_address = self.repo.create_address(&new_address).await?;
        Ok(created_address.into())
    }
    
    // Additional Update/Delete Address methods can be added similarly
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::contact_entity::Contact;
    use crate::domain::repository::contact_repository::MockContactRepository;

    #[tokio::test]
    async fn test_create_contact_success() {
        let mut mock_repo = MockContactRepository::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_create_contact()
            .times(1)
            .returning(|c| Ok(c.clone()));

        let usecase = ContactUsecase::new(Arc::new(mock_repo));

        let req = CreateContactRequest {
            first_name: "John".to_string(),
            last_name: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("123456789".to_string()),
        };

        let result = usecase.create_contact(user_id, req).await;
        assert!(result.is_ok());
        let contact = result.unwrap();
        assert_eq!(contact.first_name, "John");
    }

    #[tokio::test]
    async fn test_get_contact_unauthorized() {
        let mut mock_repo = MockContactRepository::new();
        let user_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        mock_repo
            .expect_find_contact_by_id()
            .with(mockall::predicate::eq(contact_id))
            .times(1)
            .returning(move |_| Ok(Some(Contact {
                id: Uuid::new_v4(),
                user_id: other_user_id, // Different user
                first_name: "Jane".to_string(),
                last_name: None,
                email: None,
                phone: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })));

        let usecase = ContactUsecase::new(Arc::new(mock_repo));

        let result = usecase.get_contact(user_id, contact_id).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Unauthorized");
    }
}
