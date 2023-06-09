use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewMessage {
    pub space_id: crate::model::space::SpaceId,
    pub author: String,
    pub msg_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub msg_id: Option<MessageId>,
    pub space_id: crate::model::space::SpaceId,
    pub author: String,
    pub msg_text: String,
    pub msg_time: DateTime<Utc>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageId(pub i64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewMessageCreated {
    pub uri: String,
}
