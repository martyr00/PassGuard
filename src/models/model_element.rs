use serde::{Deserialize, Serialize};

use crate::constants::{
    CCV_ELEMENT_SIZE, MAX_ADDRESS_ELEMENT_SIZE, MAX_CITY_ELEMENT_SIZE, MAX_COMPANY_ELEMENT_SIZE,
    MAX_COUNTRY_ELEMENT_SIZE, MAX_DESCRIPTION_ELEMENT_SIZE, MAX_FIRST_NAME_ELEMENT_SIZE,
    MAX_INDEX_ELEMENT_SIZE, MAX_LAST_NAME_ELEMENT_SIZE, MAX_LOGIN_ELEMENT_SIZE,
    MAX_MAIL_ELEMENT_SIZE, MAX_MONTH_CARD_ELEMENT_SIZE, MAX_NAME_ELEMENT_SIZE,
    MAX_OWNERS_NAME_ELEMENT_SIZE, MAX_PASSWORD_ELEMENT_SIZE, MAX_REGION_ELEMENT_SIZE,
    MAX_SECOND_NAME_ELEMENT_SIZE, MAX_TEL_ELEMENT_SIZE, MAX_TYPE_CARD_ELEMENT_SIZE,
    NUMBER_ELEMENT_SIZE, YEAR_CARD_ELEMENT_SIZE,
};
use crate::models::ValidDataLogin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub id_user: String,

    pub name: String,

    pub login: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,

    pub owners_name: Option<String>,
    pub number: Option<String>,
    pub type_card: Option<String>,
    pub month_card: Option<String>,
    pub year_card: Option<String>,
    pub ccv: Option<String>,

    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub last_name: Option<String>,

    pub company: Option<String>,
    pub mail: Option<String>,
    pub telephone: Option<String>,
    pub address_2: Option<String>,
    pub address_1: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub index: Option<String>,
    pub country: Option<String>,

    pub description: Option<String>,

    pub favorite: bool,
}

fn check_element(option_text: Option<&String>, len: usize) -> bool {
    match option_text {
        None => true,
        Some(text) => {
            if text.is_empty() {
                return false;
            }
            if text.len() > len {
                return false;
            }
            true
        }
    }
}

pub fn is_valid_element(element: &Element) -> ValidDataLogin {
    if element.name.len() >= MAX_NAME_ELEMENT_SIZE {
        return ValidDataLogin::BadName;
    }
    if element.name.is_empty() {
        return ValidDataLogin::BadName;
    }

    if check_element(element.login.as_ref(), MAX_LOGIN_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadLogin;
    }

    if check_element(element.password.as_ref(), MAX_PASSWORD_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadPassword;
    }

    if check_element(element.description.as_ref(), MAX_DESCRIPTION_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadDescription;
    }

    if check_element(element.owners_name.as_ref(), MAX_OWNERS_NAME_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadOwnersName;
    }

    if check_element(element.number.as_ref(), NUMBER_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadNumber;
    }

    if check_element(element.type_card.as_ref(), MAX_TYPE_CARD_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadTypeCard;
    }

    if check_element(element.month_card.as_ref(), MAX_MONTH_CARD_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadMonthCard;
    }

    if check_element(element.year_card.as_ref(), YEAR_CARD_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadYearCard;
    }

    if check_element(element.ccv.as_ref(), CCV_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadCCV;
    }

    if check_element(element.first_name.as_ref(), MAX_FIRST_NAME_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadFirstName;
    }

    if check_element(element.second_name.as_ref(), MAX_SECOND_NAME_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadSecondName;
    }

    if check_element(element.last_name.as_ref(), MAX_LAST_NAME_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadLastName;
    }

    if check_element(element.company.as_ref(), MAX_COMPANY_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadCompany;
    }

    if check_element(element.country.as_ref(), MAX_COUNTRY_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadCountry;
    }

    if check_element(element.city.as_ref(), MAX_CITY_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadCity;
    }

    if check_element(element.mail.as_ref(), MAX_MAIL_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadMail;
    }

    if check_element(element.telephone.as_ref(), MAX_TEL_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadTelephone;
    }

    if check_element(element.address_1.as_ref(), MAX_ADDRESS_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadAddress1;
    }

    if check_element(element.address_2.as_ref(), MAX_ADDRESS_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadAddress2;
    }

    if check_element(element.region.as_ref(), MAX_REGION_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadRegion;
    }

    if check_element(element.index.as_ref(), MAX_INDEX_ELEMENT_SIZE) == false {
        return ValidDataLogin::BadIndex;
    }

    return ValidDataLogin::Ok;
}
