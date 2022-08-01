use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ElementRequest {
    pub(crate) data_element: String,
    pub(crate) data_preview: String,
}