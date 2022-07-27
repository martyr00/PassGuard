use serde::Serialize;

#[derive(Serialize)]
pub struct VecErrorsElementInModel {
    pub(crate) error: Vec<ErrorElement>,
}

#[derive(Serialize)]
pub struct ErrorElement {
    pub(crate) name: &'static str,
    pub(crate) cause: &'static str,
}
