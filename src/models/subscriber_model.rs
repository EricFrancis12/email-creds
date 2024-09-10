use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub _id: ObjectId,
    pub email_list_id: ObjectId,
    pub name: String,
    pub email_addr: String,
}
