use argon2::{self, Config};
use rand::Rng;
use regex::Regex;

pub async fn register_user(
    store: crate::store::Store,
    new_user: crate::model::user::NewUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    if new_user.password.chars().count() < 8 {
        return Err(warp::reject::custom(
            crate::error::Error::IllegalArgumentException(String::from("Password too short. Use at least 8 characters")),
        ));
    }
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{1,29}$").unwrap();
    if !re.is_match(&new_user.username) {
        return Err(warp::reject::custom(
            crate::error::Error::IllegalArgumentException(String::from("Invalid username")),
        ));
    }
    match create(store, new_user).await {
        Ok(space) => Ok(warp::reply::json(&space)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn create(
    store: crate::store::Store,
    new_user: crate::model::user::NewUser,
) -> Result<crate::model::user::NewUserCreated, crate::error::Error> {
    let hashed_password = hash_password(new_user.password.as_bytes());
    match store.create_user(crate::model::user::User {
        user_id: new_user.username,
        pw_hash: hashed_password,
    }).await {
        Ok(user) => Ok(crate::model::user::NewUserCreated {
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

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}
