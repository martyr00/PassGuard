pub mod connect_to_db;
pub mod methods_for_auth_to_mondo_db;
pub mod methods_for_element_to_mongo_db;

use crate::models::tokens::Token;

pub enum LoginError {
    Ok(Token),
    WrongLogin,
    WrongPassword,
    Unknown,
}

pub enum RegistrationError {
    Ok(Token),
    AlreadyRegisteredByEmail,
    AlreadyRegisteredByLogin,
    WrongPassword,
    Unknown,
}

pub enum FindUserBy {
    UserNotFound,
    UserFoundByLogin,
    UserFoundByEmail,
}
