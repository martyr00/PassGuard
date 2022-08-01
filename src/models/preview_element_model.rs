use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preview {
    pub _id: ObjectId,
    pub user_id: String,
    pub data:Vec<u8>,
}