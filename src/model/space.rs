use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Space {
    pub space_id: Option<SpaceId>,
    pub name: String,
    pub owner: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpaceId(pub i64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewSpaceCreated {
    pub name: String,
    pub uri: String,
}
