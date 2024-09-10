use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailList {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub name: String,
}
