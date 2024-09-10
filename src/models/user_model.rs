use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
}

impl User {
    fn new(username: String, password: String) -> Self {
        User {
            _id: ObjectId::new(),
            username,
            password,
        }
    }
}
