use bcrypt::{hash, verify, DEFAULT_COST};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|e| e.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
        verify(password, hash).map_err(|e| e.to_string())
    }
}
