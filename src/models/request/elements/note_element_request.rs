use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NoteElementRequest {
    pub name: String,
    pub description: String,

    pub folder: Option<String>,
    pub favourite: bool,
}
