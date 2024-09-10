use mongodb::{
    bson::{doc, oid::ObjectId},
    options::UpdateModifications,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreationRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        User {
            _id: ObjectId::new(),
            username,
            password,
        }
    }

    pub fn new_from(creation_req: UserCreationRequest) -> Self {
        User::new(creation_req.username, creation_req.password)
    }
}

impl UserUpdateRequest {
    pub fn to_doc(&self) -> UpdateModifications {
        let mut updates = doc! {};

        if let Some(username) = &self.username {
            updates.insert("username", username);
        }

        if let Some(password) = &self.password {
            updates.insert("password", password);
        }

        UpdateModifications::Document(doc! { "$set": updates })
    }
}
