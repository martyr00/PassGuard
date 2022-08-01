use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseForPostElement {
    pub user_id: String,
    pub element:Vec<u8>,
    pub preview:Vec<u8>,
}