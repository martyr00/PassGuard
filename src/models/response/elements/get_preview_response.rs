use crate::models::preview_element_model::Preview;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseGetPreview {
    pub data: Vec<Preview>,
}
