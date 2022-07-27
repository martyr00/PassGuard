use crate::models::elements_model::Element;
use crate::models::response::elements::error_elements::ErrorElement;
use crate::routes::elements::post_elements::PostElementError;

const ERROR_NAME_LEN: ErrorElement = ErrorElement {
    name: "name",
    cause: "name len limits 1..200",
};
const ERROR_FIRST_NAME_LEN: ErrorElement = ErrorElement {
    name: "first_name",
    cause: "first_name len limits 1..500",
};
const ERROR_SECOND_NAME_LEN: ErrorElement = ErrorElement {
    name: "second_name",
    cause: "second_name len limits 1..500",
};
const ERROR_LAST_NAME_LEN: ErrorElement = ErrorElement {
    name: "last_name",
    cause: "last_name len limits 1..500",
};
const ERROR_FOLDER_LEN: ErrorElement = ErrorElement {
    name: "folder",
    cause: "folder len limits 1..200",
};
const ERROR_COMPANY_LEN: ErrorElement = ErrorElement {
    name: "company",
    cause: "company len limits 1..500",
};
const ERROR_CITY_LEN: ErrorElement = ErrorElement {
    name: "city",
    cause: "company len limits 1..500",
};
const ERROR_CCV_LEN: ErrorElement = ErrorElement {
    name: "ccv",
    cause: "company len limits 3..3",
};
const ERROR_COUNTRY_LEN: ErrorElement = ErrorElement {
    name: "country",
    cause: "company len limits 1..500",
};
const ERROR_DESCRIPTION_LEN: ErrorElement = ErrorElement {
    name: "description",
    cause: "company len limits 1..500",
};
const ERROR_LOGIN_LEN: ErrorElement = ErrorElement {
    name: "login",
    cause: "company len limits 1..500",
};
const ERROR_MAIL_LEN: ErrorElement = ErrorElement {
    name: "mail",
    cause: "company len limits 1..500",
};
const ERROR_MONTH_CARD_LEN: ErrorElement = ErrorElement {
    name: "month_card",
    cause: "company len limits 1..2",
};
const ERROR_OWNERS_NAME_LEN: ErrorElement = ErrorElement {
    name: "owners_name",
    cause: "second_name len limits 1..500",
};
const ERROR_YEAR_CARD_LEN: ErrorElement = ErrorElement {
    name: "year_card",
    cause: "company len limits 4..4",
};
const ERROR_TYPE_CARD_LEN: ErrorElement = ErrorElement {
    name: "type_card",
    cause: "company len limits 1..5",
};
const ERROR_PASSWORD_LEN: ErrorElement = ErrorElement {
    name: "password",
    cause: "company len limits 1..500",
};
const ERROR_ADDRESS_1_LEN: ErrorElement = ErrorElement {
    name: "address_1",
    cause: "company len limits 1..500",
};
const ERROR_ADDRESS_2_LEN: ErrorElement = ErrorElement {
    name: "address_2",
    cause: "company len limits 1..500",
};
const ERROR_INDEX_LEN: ErrorElement = ErrorElement {
    name: "index",
    cause: "company len limits 1..200",
};

const MIN_LEN: usize = 1;

pub fn is_valid_element(element: &Element) -> PostElementError {
    let mut vec_errors: Vec<ErrorElement> = Vec::new();
    if is_valid_text_element(&Some(element.name.clone()), 200, MIN_LEN) == false {
        vec_errors.push(ERROR_NAME_LEN)
    }
    if is_valid_text_element(&element.last_name, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_LAST_NAME_LEN)
    }
    if is_valid_text_element(&element.second_name, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_SECOND_NAME_LEN)
    }
    if is_valid_text_element(&element.first_name, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_FIRST_NAME_LEN)
    }
    if is_valid_text_element(&element.folder, 200, MIN_LEN) == false {
        vec_errors.push(ERROR_FOLDER_LEN)
    }
    if is_valid_text_element(&element.company, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_COMPANY_LEN)
    }
    if is_valid_text_element(&element.city, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_CITY_LEN)
    }
    if is_valid_text_element(&element.ccv, 3, 3) == false {
        vec_errors.push(ERROR_CCV_LEN)
    }
    if is_valid_text_element(&element.country, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_COUNTRY_LEN)
    }
    if is_valid_text_element(&element.description, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_DESCRIPTION_LEN)
    }
    if is_valid_text_element(&element.login, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_LOGIN_LEN)
    }
    if is_valid_text_element(&element.mail, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_MAIL_LEN)
    }
    if is_valid_text_element(&element.month_card, 2, MIN_LEN) == false {
        vec_errors.push(ERROR_MONTH_CARD_LEN)
    }
    if is_valid_text_element(&element.owners_name, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_OWNERS_NAME_LEN)
    }
    if is_valid_text_element(&element.year_card, 4, MIN_LEN) == false {
        vec_errors.push(ERROR_YEAR_CARD_LEN)
    }
    if is_valid_text_element(&element.password, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_PASSWORD_LEN)
    }
    if is_valid_text_element(&element.type_card, 10, MIN_LEN) == false {
        vec_errors.push(ERROR_TYPE_CARD_LEN)
    }
    if is_valid_text_element(&element.address_1, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_ADDRESS_1_LEN)
    }
    if is_valid_text_element(&element.address_2, 500, MIN_LEN) == false {
        vec_errors.push(ERROR_ADDRESS_2_LEN)
    }
    if is_valid_text_element(&element.index, 200, MIN_LEN) == false {
        vec_errors.push(ERROR_INDEX_LEN)
    }

    if !vec_errors.is_empty() {
        return PostElementError::ErrorElements(vec_errors);
    }
    PostElementError::Ok
}

fn is_valid_text_element(option_text: &Option<String>, max: usize, min: usize) -> bool {
    match option_text {
        None => true,
        Some(text) => {
            if text.len() <= min || text.len() >= max {
                return false;
            }
            return true;
        }
    }
}
