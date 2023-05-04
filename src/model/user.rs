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
        let mut is_same_owner = false;
        if let Some(username) = &self.username {
            is_same_owner = username == pretend_user;
        } else {
            return Some(crate::error::Error::AuthenticationError(String::from(
                message,
            )));
        }
        if !is_same_owner {
            return Some(crate::error::Error::AuthorizationError(String::from(
                message,
            )));
        }
        None
    }
}
