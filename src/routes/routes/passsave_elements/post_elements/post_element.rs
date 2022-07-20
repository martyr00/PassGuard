use crate::database::connect_to_db::MongoDB;
use crate::models::model_element::{is_valid_element, Element};
use crate::models::ValidDataLogin;
use crate::routes::routes::passsave_elements::post_elements::ErrorPostElement;
use rocket::State;

pub async fn post_element(element: &Element, database: &State<MongoDB>) -> ErrorPostElement {
    match is_valid_element(&element) {
        ValidDataLogin::Ok => match database.post_element(&element).await {
            Ok(_) => ErrorPostElement::Ok,
            Err(_) => ErrorPostElement::Unknown,
        },
        ValidDataLogin::BadName => ErrorPostElement::BadName,
        ValidDataLogin::BadLogin => ErrorPostElement::BadLogin,
        ValidDataLogin::BadPassword => ErrorPostElement::BadPassword,
        ValidDataLogin::BadOwnersName => ErrorPostElement::BadOwnersName,
        ValidDataLogin::BadNumber => ErrorPostElement::BadNumber,
        ValidDataLogin::BadTypeCard => ErrorPostElement::BadTypeCard,
        ValidDataLogin::BadMonthCard => ErrorPostElement::BadMonthCard,
        ValidDataLogin::BadYearCard => ErrorPostElement::BadYearCard,
        ValidDataLogin::BadCCV => ErrorPostElement::BadCCV,
        ValidDataLogin::BadFirstName => ErrorPostElement::BadFirstName,
        ValidDataLogin::BadSecondName => ErrorPostElement::BadSecondName,
        ValidDataLogin::BadLastName => ErrorPostElement::BadLastName,
        ValidDataLogin::BadCompany => ErrorPostElement::BadCompany,
        ValidDataLogin::BadMail => ErrorPostElement::BadMail,
        ValidDataLogin::BadAddress1 => ErrorPostElement::BadAddress1,
        ValidDataLogin::BadAddress2 => ErrorPostElement::BadAddress2,
        ValidDataLogin::BadCity => ErrorPostElement::BadCity,
        ValidDataLogin::BadRegion => ErrorPostElement::BadRegion,
        ValidDataLogin::BadIndex => ErrorPostElement::BadIndex,
        ValidDataLogin::BadCountry => ErrorPostElement::BadCountry,
        ValidDataLogin::BadDescription => ErrorPostElement::BadDescription,
        ValidDataLogin::BadTelephone => ErrorPostElement::BadTelephone,
    }
}
