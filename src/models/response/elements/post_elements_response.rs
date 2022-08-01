use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseForPostElement {
    pub user_id: String,
    pub element:String,
    pub preview:String,
}