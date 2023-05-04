use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Extension, Json};
use regex::Regex;

use crate::model::{
    message::{Message, NewMessage, NewMessageCreated},
    user::Session,
};

pub async fn create_message(
    State(store): State<Arc<crate::store::Store>>,
    Extension(current_session): Extension<Session>,
    Json(new_message): Json<NewMessage>,
) -> impl IntoResponse {
    if new_message.msg_text.chars().count() > 1024 {
        return Err(crate::error::Error::IllegalArgumentException(String::from(
            "Message text too long",
        )));
    }
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{1,29}$").unwrap();
    if !re.is_match(&new_message.author) {
        return Err(crate::error::Error::IllegalArgumentException(String::from(
            "Invalid author",
        )));
    }
    if let Some(value) = current_session
        .get_error_if_user_not_match(&new_message.author, "Author must match authenticated user")
    {
        return Err(value);
    }
    match create(
        store,
        Message {
            msg_id: None,
            space_id: new_message.space_id,
            author: new_message.author,
            msg_text: new_message.msg_text,
            msg_time: chrono::Utc::now(),
        },
    )
    .await
    {
        Ok(message) => Ok(Json(message)),
        Err(e) => Err(e),
    }
}

async fn create(
    store: Arc<crate::store::Store>,
    new_message: Message,
) -> Result<NewMessageCreated, crate::error::Error> {
    match store.create_message(new_message).await {
        Ok(message) => Ok(NewMessageCreated {
            uri: format!("/messages/{}", &message.msg_id.unwrap().0),
        }),
        Err(e) => Err(e),
    }
}
