use crate::models::response::elements::error_elements::ErrorElement;

pub mod post_card;
pub mod post_login;
pub mod post_note;
pub mod post_personal_inf;

pub enum PostElementError {
    Ok,
    ErrorElements(Vec<ErrorElement>),
}
