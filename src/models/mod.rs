pub mod hello_response;
pub mod model_element;
pub mod model_user;
pub mod request;
pub mod response;
pub mod tokens;

pub enum ValidDataLogin {
    Ok,
    BadName,
    BadLogin,
    BadPassword,
    BadOwnersName,
    BadNumber,
    BadTypeCard,
    BadMonthCard,
    BadYearCard,
    BadCCV,
    BadFirstName,
    BadSecondName,
    BadLastName,
    BadCompany,
    BadMail,
    BadAddress1,
    BadAddress2,
    BadCity,
    BadRegion,
    BadIndex,
    BadCountry,
    BadDescription,
    BadTelephone,
}
