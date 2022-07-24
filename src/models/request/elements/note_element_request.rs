use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NoteElementRequest {
    pub name: String,
    pub description: Option<String>,

    pub folder: Option<String>,
    pub favourite: bool,
}
