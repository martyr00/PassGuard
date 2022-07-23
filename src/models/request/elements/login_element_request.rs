use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginElementRequest {
    pub name: String,
    pub description: String,

    pub login: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,

    pub folder: Option<String>,
    pub favourite: bool,
}
