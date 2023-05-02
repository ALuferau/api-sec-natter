use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Extension, Json};
use regex::Regex;

use crate::model::user::Session;

pub async fn create_space(
    State(store): State<Arc<crate::store::Store>>,
    Extension(current_session): Extension<Session>,
    Json(new_space): Json<crate::model::space::Space>,
) -> impl IntoResponse {
    if new_space.name.chars().count() > 255 {
        return Err(crate::error::Error::IllegalArgumentException(String::from(
            "Space name too long",
        )));
    }
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{1,29}$").unwrap();
    if !re.is_match(&new_space.owner) {
        return Err(crate::error::Error::IllegalArgumentException(String::from(
            "Invalid username",
        )));
    }
    let mut is_same_owner = false;
    if let Some(username) = current_session.username {
        is_same_owner = username == new_space.owner;
    }
    if !is_same_owner {
        return Err(crate::error::Error::IllegalArgumentException(String::from(
            "Owner must match authenticated user",
        )));
    }
    match create(store, new_space).await {
        Ok(space) => Ok(Json(space)),
        Err(e) => Err(e),
    }
}

async fn create(
    store: Arc<crate::store::Store>,
    new_space: crate::model::space::Space,
) -> Result<crate::model::space::NewSpaceCreated, crate::error::Error> {
    match store.create_space(new_space).await {
        Ok(space) => Ok(crate::model::space::NewSpaceCreated {
            name: format!("{}", &space.name),
            uri: format!("/spaces/{}", &space.space_id.unwrap().0),
        }),
        Err(e) => Err(e),
    }
}
