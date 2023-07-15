use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub pw_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserCreated {
    pub username: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub username: Option<String>,
}

impl Session {
    pub fn get_error_if_user_not_match(
        &self,
        pretend_user: &str,
        message: &str,
    ) -> Option<crate::error::Error> {
        match &self.username {
            Some(username) if username == pretend_user => None,
            Some(_) => Some(crate::error::Error::AuthorizationError(String::from(
                message,
            ))),
            None => Some(crate::error::Error::AuthenticationError(String::from(
                message,
            ))),
        }
    }
}
