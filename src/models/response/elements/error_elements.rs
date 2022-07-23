use serde::Serialize;

#[derive(Serialize)]
pub struct VecErrorsElementInModel {
    pub(crate) error: Vec<ErrorElement>,
}

pub struct ErrorElement {
    name: String,
    cause: String,
}
