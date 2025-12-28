use super::super::entity::address_entity::Address;
use super::super::entity::contact_entity::Contact;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn create_contact(&self, contact: &Contact) -> Result<Contact, String>;
    async fn update_contact(&self, contact: &Contact) -> Result<Contact, String>;
    async fn delete_contact(&self, id: &Uuid) -> Result<(), String>;
    async fn find_contact_by_id(&self, id: &Uuid) -> Result<Option<Contact>, String>;
    async fn find_contacts_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Contact>, String>;

    // Address operations (nested in ContactRepository for simplicity as requested)
    // Or we can assume addresses are loaded with contacts if needed, or separate methods.
    // For this requirement "in every contact saved address", let's include address ops or relation.
    // We will separate creating address to keep it flexible.
    
    async fn create_address(&self, address: &Address) -> Result<Address, String>;
    async fn update_address(&self, address: &Address) -> Result<Address, String>;
    async fn delete_address(&self, id: &Uuid) -> Result<(), String>;
    async fn find_address_by_id(&self, id: &Uuid) -> Result<Option<Address>, String>;
    async fn find_addresses_by_contact_id(&self, contact_id: &Uuid) -> Result<Vec<Address>, String>;
}
