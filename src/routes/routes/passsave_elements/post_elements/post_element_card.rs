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
use crate::models::model_element::Element;
use crate::models::request::elements::model_card::DataElementCard;
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::routes::routes::passsave_elements::post_elements::post_element::post_element;
use crate::routes::routes::passsave_elements::post_elements::ErrorPostElement;
use crate::{ErrorResponse, Status, UNKNOWN};

#[post(
    "/element/card",
    data = "<option_data_element_card_model>",
    format = "json"
)]
pub async fn post_element_card(
    auth: AuthorizedUser,
    option_data_element_card_model: Option<Json<DataElementCard>>,
    database: &State<MongoDB>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {
    match option_data_element_card_model {
        None => Err(WRONG_REQUEST),
        Some(element_card) => {
            let element = Element {
                id_user: auth.user_id,
                name: element_card.name.clone(),
                login: None,
                password: None,
                url: None,
                owners_name: element_card.owners_name.clone(),
                number: element_card.number.clone(),
                type_card: element_card.type_card.clone(),
                month_card: element_card.month_card.clone(),
                year_card: element_card.year_card.clone(),
                ccv: element_card.ccv.clone(),
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
                description: element_card.description.clone(),
                favorite: element_card.favorite,
            };
            match post_element(&element, database).await {
                ErrorPostElement::Ok => Ok((Status::Ok, Json(element))),
                ErrorPostElement::Unknown => Err(UNKNOWN),
                ErrorPostElement::BadName => Err(ERROR_NAME_ELEMENT_SIZE),
                ErrorPostElement::BadLogin => Err(ERROR_LOGIN_ELEMENT_SIZE),
                ErrorPostElement::BadPassword => Err(ERROR_PASSWORD_ELEMENT_SIZE),
                ErrorPostElement::BadOwnersName => Err(ERROR_OWNERS_NAME_ELEMENT_SIZE),
                ErrorPostElement::BadNumber => Err(ERROR_NUMBER_ELEMENT_SIZE),
                ErrorPostElement::BadTypeCard => Err(ERROR_TYPE_CARD_ELEMENT_SIZE),
                ErrorPostElement::BadMonthCard => Err(ERROR_MONTH_CARD_ELEMENT_SIZE),
                ErrorPostElement::BadYearCard => Err(ERROR_YEAR_CARD_ELEMENT_SIZE),
                ErrorPostElement::BadCCV => Err(ERROR_CCV_ELEMENT_SIZE),
                ErrorPostElement::BadFirstName => Err(ERROR_FIRST_NAME_ELEMENT_SIZE),
                ErrorPostElement::BadSecondName => Err(ERROR_SECOND_NAME_ELEMENT_SIZE),
                ErrorPostElement::BadLastName => Err(ERROR_LAST_NAME_ELEMENT_SIZE),
                ErrorPostElement::BadCompany => Err(ERROR_COMPANY_ELEMENT_SIZE),
                ErrorPostElement::BadMail => Err(ERROR_MAIL_ELEMENT_SIZE),
                ErrorPostElement::BadAddress1 => Err(ERROR_ADDRESS_ELEMENT_SIZE),
                ErrorPostElement::BadAddress2 => Err(ERROR_ADDRESS_ELEMENT_SIZE),
                ErrorPostElement::BadCity => Err(ERROR_CITY_ELEMENT_SIZE),
                ErrorPostElement::BadRegion => Err(ERROR_REGION_ELEMENT_SIZE),
                ErrorPostElement::BadIndex => Err(ERROR_INDEX_ELEMENT_SIZE),
                ErrorPostElement::BadCountry => Err(ERROR_COUNTRY_ELEMENT_SIZE),
                ErrorPostElement::BadDescription => Err(ERROR_DESCRIPTION_ELEMENT_SIZE),
                ErrorPostElement::BadTelephone => Err(ERROR_TEL_ELEMENT_SIZE),
            }
        }
    }
}
