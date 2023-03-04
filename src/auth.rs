use crate::Permissions;

pub fn check_key(key: &str) -> Option<Token> {
    todo!("check_key");
}
pub fn create_token(key: &str, permissions: Permissions::Permissions) -> Result<Token, String> {
    todo!("create_token")
}

pub fn remove_key() -> bool {
    todo!("remove_key")
}

pub struct Token {
    pub token: String,
}
