use rocket::serde::json::Json;
use rocket::State;

use crate::helper::{get_valid_desc, ValidDescAndUrlError, object_id_parse_str};
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::routes::validator_authorization::{get_valid_login_and_password};
use crate::{ErrorResponse, Status, UNKNOWN};
use crate::constants::{LEN_DESCRIPTION_IN_ELEMENT_LOGIN, LEN_LOGIN_IN_ELEMENT_LOGIN, LEN_PASSWORD_IN_ELEMENT_LOGIN, WEAK_LOGIN, WEAK_PASSWORD, WRONG_DESCRIPTION, WRONG_REQUEST};
use crate::database::connect_to_db::MongoDB;
use crate::models::request::model_login::DataElementLogin;
use crate::routes::TypeValidTwoStr;

#[post("/element", data = "<option_data_element_login_model>", format = "json")]
pub async fn post_element_login(
    auth: AuthorizedUser,
    option_data_element_login_model: Option<Json<DataElementLogin>>,
    database: &State<MongoDB>,
) -> Result<Status, (Status, Json<ErrorResponse>)> {
    match object_id_parse_str(auth.user_id) {
        Ok(id) => {
            match check_data_element_login(option_data_element_login_model) {
                DataElementLoginError::Ok(element_login ) => {
                    match database.add_element_login(element_login, id).await {
                        Ok(_) => Ok(Status::Ok),
                        Err(_) => Err(UNKNOWN),
                    }},
                DataElementLoginError::BadLogin => { Err(WEAK_LOGIN)},
                DataElementLoginError::BadPassword => { Err(WEAK_PASSWORD)},
                DataElementLoginError::BadDescription => { Err(WRONG_DESCRIPTION)},
                DataElementLoginError::NoneElement => { Err(WRONG_REQUEST)},
            }
        },
        Err(_) => {Err(UNKNOWN)}
    }

}

fn check_data_element_login(option_element: Option<Json<DataElementLogin>>) -> DataElementLoginError {
    match option_element {
        None => { DataElementLoginError::NoneElement },
        Some(element) => {
            match valid_data_element_login(element) {
                ValidDataElementLoginError::Ok(element_is_valid) => {DataElementLoginError::Ok(element_is_valid)}
                ValidDataElementLoginError::BadLogin => {DataElementLoginError::BadLogin}
                ValidDataElementLoginError::BadPassword => {DataElementLoginError::BadPassword}
                ValidDataElementLoginError::BadDescription => {DataElementLoginError::BadDescription}
            }
        }
    }
}

fn valid_data_element_login(element: Json<DataElementLogin>) -> ValidDataElementLoginError {
    match get_valid_login_and_password(
        &element.login,
        &element.password,
        LEN_LOGIN_IN_ELEMENT_LOGIN,
        LEN_PASSWORD_IN_ELEMENT_LOGIN
    ) {
        TypeValidTwoStr::Ok => {
            match get_valid_desc(element.description.clone(), LEN_DESCRIPTION_IN_ELEMENT_LOGIN) {
                ValidDescAndUrlError::Ok => {ValidDataElementLoginError::Ok(element)},
                ValidDescAndUrlError::DescriptionIsNotValid => {ValidDataElementLoginError::BadDescription},
                ValidDescAndUrlError::NoneDescription => {ValidDataElementLoginError::BadDescription}
            }
        },
        TypeValidTwoStr::BadFirst => {ValidDataElementLoginError::BadLogin},
        TypeValidTwoStr::BadSecond => {ValidDataElementLoginError::BadPassword},
    }
}


pub enum ValidDataElementLoginError {
    Ok(Json<DataElementLogin>),
    BadLogin,
    BadPassword,
    BadDescription
}

pub enum DataElementLoginError {
    Ok(Json<DataElementLogin>),
    BadLogin,
    BadPassword,
    BadDescription,
    NoneElement,
}
