use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataElementNote {
    pub name: String,

    pub description: Option<String>,
    pub favorite: bool,
}
