use crate::models::elements_model::Element;
use crate::models::response::elements::error_elements::ErrorElement;
use crate::routes::elements::post_elements::post_note::ErrorElement;

pub mod post_card;
pub mod post_login;
pub mod post_note;
pub mod post_personal_inf;

pub enum PostElementError {
    Ok,
    Unknown,
    ErrorElements(Vec<ErrorElement>),
}

pub struct PostElementRequestError {
    name: String,
    cause: String,
}
