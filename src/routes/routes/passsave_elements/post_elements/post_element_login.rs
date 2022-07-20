use rocket::serde::json::Json;
use rocket::State;

use crate::constants::{
    ERROR_ADDRESS_ELEMENT_SIZE, ERROR_CCV_ELEMENT_SIZE, ERROR_CITY_ELEMENT_SIZE,
    ERROR_COMPANY_ELEMENT_SIZE, ERROR_COUNTRY_ELEMENT_SIZE, ERROR_DESCRIPTION_ELEMENT_SIZE,
    ERROR_FIRST_NAME_ELEMENT_SIZE, ERROR_INDEX_ELEMENT_SIZE, ERROR_LAST_NAME_ELEMENT_SIZE,
    ERROR_LOGIN_ELEMENT_SIZE, ERROR_MAIL_ELEMENT_SIZE, ERROR_MONTH_CARD_ELEMENT_SIZE,
    ERROR_NAME_ELEMENT_SIZE, ERROR_NUMBER_ELEMENT_SIZE, ERROR_OWNERS_NAME_ELEMENT_SIZE,
    ERROR_PASSWORD_ELEMENT_SIZE, ERROR_REGION_ELEMENT_SIZE, ERROR_SECOND_NAME_ELEMENT_SIZE,
    ERROR_TEL_ELEMENT_SIZE, ERROR_TYPE_CARD_ELEMENT_SIZE, ERROR_YEAR_CARD_ELEMENT_SIZE,
    WRONG_REQUEST,
};
use crate::database::connect_to_db::MongoDB;
use crate::models::model_element::{is_valid_element, Element};
use crate::models::request::elements::model_login::DataElementLogin;
use crate::models::ValidDataLogin;
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::{ErrorResponse, Status, UNKNOWN};

#[post(
    "/element/login",
    data = "<option_data_element_login_model>",
    format = "json"
)]
pub async fn post_element_login(
    auth: AuthorizedUser,
    option_data_element_login_model: Option<Json<DataElementLogin>>,
    database: &State<MongoDB>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {
    match option_data_element_login_model {
        None => Err(WRONG_REQUEST),
        Some(element_login) => {
            let element = Element {
                id_user: auth.user_id,
                name: element_login.name.clone(),
                login: Some(element_login.login.clone()),
                password: Some(element_login.password.clone()),
                url: element_login.url.clone(),
                owners_name: None,
                number: None,
                type_card: None,
                month_card: None,
                year_card: None,
                ccv: None,
                first_name: None,
                second_name: None,
                last_name: None,
                company: None,
                mail: None,
                telephone: None,
                address_2: None,
                address_1: None,
                city: None,
                region: None,
                index: None,
                country: None,
                description: element_login.description.clone(),
                favorite: element_login.favorite,
            };
            match is_valid_element(&element) {
                ValidDataLogin::Ok => match database.post_element(&element).await {
                    Ok(_) => Ok((Status::Ok, Json(element))),
                    Err(_) => Err(UNKNOWN),
                },
                ValidDataLogin::BadName => Err(ERROR_NAME_ELEMENT_SIZE),
                ValidDataLogin::BadLogin => Err(ERROR_LOGIN_ELEMENT_SIZE),
                ValidDataLogin::BadPassword => Err(ERROR_PASSWORD_ELEMENT_SIZE),
                ValidDataLogin::BadOwnersName => Err(ERROR_OWNERS_NAME_ELEMENT_SIZE),
                ValidDataLogin::BadNumber => Err(ERROR_NUMBER_ELEMENT_SIZE),
                ValidDataLogin::BadTypeCard => Err(ERROR_TYPE_CARD_ELEMENT_SIZE),
                ValidDataLogin::BadMonthCard => Err(ERROR_MONTH_CARD_ELEMENT_SIZE),
                ValidDataLogin::BadYearCard => Err(ERROR_YEAR_CARD_ELEMENT_SIZE),
                ValidDataLogin::BadCCV => Err(ERROR_CCV_ELEMENT_SIZE),
                ValidDataLogin::BadFirstName => Err(ERROR_FIRST_NAME_ELEMENT_SIZE),
                ValidDataLogin::BadSecondName => Err(ERROR_SECOND_NAME_ELEMENT_SIZE),
                ValidDataLogin::BadLastName => Err(ERROR_LAST_NAME_ELEMENT_SIZE),
                ValidDataLogin::BadCompany => Err(ERROR_COMPANY_ELEMENT_SIZE),
                ValidDataLogin::BadMail => Err(ERROR_MAIL_ELEMENT_SIZE),
                ValidDataLogin::BadAddress1 => Err(ERROR_ADDRESS_ELEMENT_SIZE),
                ValidDataLogin::BadAddress2 => Err(ERROR_ADDRESS_ELEMENT_SIZE),
                ValidDataLogin::BadCity => Err(ERROR_CITY_ELEMENT_SIZE),
                ValidDataLogin::BadRegion => Err(ERROR_REGION_ELEMENT_SIZE),
                ValidDataLogin::BadIndex => Err(ERROR_INDEX_ELEMENT_SIZE),
                ValidDataLogin::BadCountry => Err(ERROR_COUNTRY_ELEMENT_SIZE),
                ValidDataLogin::BadDescription => Err(ERROR_DESCRIPTION_ELEMENT_SIZE),
                ValidDataLogin::BadTelephone => Err(ERROR_TEL_ELEMENT_SIZE),
            }
        }
    }
}
