use crate::models::request::login_user_request::LoginRequest;
use crate::models::request::registration_user_request::RegistrationRequest;
use rocket::serde::json::Json;
use crate::models::request::patch_user_request::EditUserRequest;

pub mod login;
pub mod registration;
pub mod token;
pub mod delete_user;
pub mod get_data_user;
pub mod patch_user;

pub enum RegistrationRequestError {
    Ok(Json<RegistrationRequest>),
    NoneRegistrationRequest,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
}

pub enum LoginRequestError {
    Ok(Json<LoginRequest>),
    NoneLoginRequest,
    BadLogin,
    BadPassword,
}

pub enum EditUserRequestError {
    Ok(Json<EditUserRequest>),
    NoneEditModel,
    BadMail,
    BadLogin,
    BadFirstName,
    BadLastName,
}

