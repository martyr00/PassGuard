mod post_element;
pub mod post_element_card;
pub mod post_element_login;
pub mod post_element_note;
pub mod post_element_personal;

pub enum ErrorPostElement {
    Ok,
    Unknown,
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
