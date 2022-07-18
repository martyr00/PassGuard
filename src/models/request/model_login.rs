use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataElementLogin {
    pub name: String,
    pub login: String,
    pub password: String,
    pub url: Option<String>,

    pub description: Option<String>,
    pub favorite: bool,
}
