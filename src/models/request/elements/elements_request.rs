use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ElementRequest {
    pub(crate) data_element: Vec<u8>,
    pub(crate) data_preview: Vec<u8>,
}