use argon2::{self, Config};
use axum::{
    extract::State, http, middleware::Next, response::IntoResponse, response::Response, Json,
};
use hyper::Request;
use rand::Rng;
use regex::Regex;
use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};

use crate::{
    error::Error,
    model::user::{NewUser, NewUserCreated, Session, User},
};

pub async fn register_user(
    State(store): State<Arc<crate::store::Store>>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    if new_user.password.chars().count() < 8 {
        return Err(Error::IllegalArgumentException(String::from(
            "Password too short. Use at least 8 characters",
        )));
    }
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{1,29}$").unwrap();
    if !re.is_match(&new_user.username) {
        return Err(Error::IllegalArgumentException(String::from(
            "Invalid username",
        )));
    }
    match create(store, new_user).await {
        Ok(new_user) => Ok(Json(new_user)),
        Err(e) => Err(e),
    }
}

async fn create(
    store: Arc<crate::store::Store>,
    new_user: NewUser,
) -> Result<NewUserCreated, crate::error::Error> {
    let hashed_password = hash_password(new_user.password.as_bytes());
    match store
        .create_user(User {
            user_id: new_user.username,
            pw_hash: hashed_password,
        })
        .await
    {
        Ok(user) => Ok(NewUserCreated {
            username: format!("{}", &user.user_id),
            uri: format!("/users/{}", &user.user_id),
        }),
        Err(e) => Err(e),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, crate::error::Error> {
    match argon2::verify_encoded(hash, password) {
        Ok(verified) => Ok(verified),
        Err(_) => Err(Error::IllegalArgumentException(String::from(
            "Invalid password or username",
        ))),
    }
}

pub async fn authenticate<B>(
    State(store): State<Arc<crate::store::Store>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    let auth_header = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let user_id = auth_and_unwrap_user_id(auth_header, store)
        .await
        .unwrap_or_else(|_| None);
    request
        .extensions_mut()
        .insert(Session { username: user_id });

    let response = next.run(request).await;
    response
}

async fn auth_and_unwrap_user_id(
    auth_header: Option<&str>,
    store: Arc<crate::store::Store>,
) -> Result<Option<String>, crate::error::Error> {
    let (id, password) = extract_credentials(auth_header)?;
    let user = store.get_user_by_id(&id).await?;
    let verified = verify_password(&user.pw_hash, password.as_bytes())?;

    if verified {
        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn extract_credentials(auth_header: Option<&str>) -> Result<(String, String), crate::error::Error> {
    let error_msg = String::from("Invalid auth token");
    if auth_header == None || !auth_header.unwrap().starts_with("Basic ") {
        Err(Error::IllegalArgumentException(error_msg))
    } else {
        let split = auth_header.unwrap().split_once(' ');
        match split {
            Some((name, contents)) if name == "Basic" => {
                let decoded = general_purpose::STANDARD
                    .decode(contents)
                    .map_err(|_| Error::IllegalArgumentException(error_msg.clone()))?;
                let decoded = String::from_utf8(decoded)
                    .map_err(|_| Error::IllegalArgumentException(error_msg.clone()))?;

                if let Some((id, password)) = decoded.split_once(':') {
                    Ok((id.to_string(), password.to_string()))
                } else {
                    Err(Error::IllegalArgumentException(error_msg.clone()))
                }
            }
            _ => Err(Error::IllegalArgumentException(error_msg.clone())),
        }
    }
}
