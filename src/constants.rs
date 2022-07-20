use crate::error_response::error_responses::{
    ErrorResponse, ALREADY_REGISTERED_EMAIL_JSON, ALREADY_REGISTERED_LOGIN_JSON,
    ERROR_ALREADY_REGISTERED_STATUS, ERROR_DESCRIPTION_STATUS, ERROR_NOT_FOUND_STATUS,
    ERROR_UNAUTHORIZED_STATUS, ERROR_UNKNOWN_STATUS, ERROR_WEAK_LOGIN_STATUS,
    ERROR_WEAK_PASSWORD_STATUS, ERROR_WRONG_FIRST_NAME_STATUS, ERROR_WRONG_LAST_NAME_STATUS,
    ERROR_WRONG_MAIL_STATUS, ERROR_WRONG_REQUEST_STATUS, NOT_FOUND_JSON, UNAUTHORIZED_JSON,
    UNKNOWN_JSON, WEAK_LOGIN_JSON, WEAK_PASSWORD_JSON, WRONG_DESCRIPTION_JSON,
    WRONG_FIRST_NAME_JSON, WRONG_LAST_NAME_JSON, WRONG_MAIL_JSON, WRONG_REQUEST_JSON,
};
use rocket::http::Status;
use rocket::serde::json::Json;

pub const EXPIRATION_REFRESH_TOKEN: i64 = 3600 * 24 * 30;
pub const EXPIRATION_TOKEN: i64 = 3600;

pub struct LenText {
    pub(crate) min: usize,
    pub(crate) max: usize,
}

//errors
pub const WRONG_REQUEST: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON));

pub const WRONG_MAIL: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_MAIL_STATUS, Json(WRONG_MAIL_JSON));

pub const ALREADY_REGISTERED_LOGIN: (Status, Json<ErrorResponse>) = (
    ERROR_ALREADY_REGISTERED_STATUS,
    Json(ALREADY_REGISTERED_LOGIN_JSON),
);

pub const ALREADY_REGISTERED_MAIL: (Status, Json<ErrorResponse>) = (
    ERROR_ALREADY_REGISTERED_STATUS,
    Json(ALREADY_REGISTERED_EMAIL_JSON),
);

pub const WEAK_PASSWORD: (Status, Json<ErrorResponse>) =
    (ERROR_WEAK_PASSWORD_STATUS, Json(WEAK_PASSWORD_JSON));

pub const WEAK_LOGIN: (Status, Json<ErrorResponse>) =
    (ERROR_WEAK_LOGIN_STATUS, Json(WEAK_LOGIN_JSON));

pub const WRONG_DESCRIPTION: (Status, Json<ErrorResponse>) =
    (ERROR_DESCRIPTION_STATUS, Json(WRONG_DESCRIPTION_JSON));

pub const UNKNOWN: (Status, Json<ErrorResponse>) = (ERROR_UNKNOWN_STATUS, Json(UNKNOWN_JSON));

pub const UNAUTHORIZED: (Status, Json<ErrorResponse>) =
    (ERROR_UNAUTHORIZED_STATUS, Json(UNAUTHORIZED_JSON));

pub const WRONG_FIRST_NAME: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_FIRST_NAME_STATUS, Json(WRONG_FIRST_NAME_JSON));

pub const WRONG_LAST_NAME: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_LAST_NAME_STATUS, Json(WRONG_LAST_NAME_JSON));

pub const NOT_FOUND: (Status, Json<ErrorResponse>) = (ERROR_NOT_FOUND_STATUS, Json(NOT_FOUND_JSON));

pub const NAME_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів  500",
    option: Some("name"),
};
pub const ERROR_NAME_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(NAME_ELEMENT_SIZE_JSON));

pub const LOGIN_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500 ",
    option: Some("login"),
};
pub const ERROR_LOGIN_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(LOGIN_ELEMENT_SIZE_JSON));

pub const PASSWORD_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500 ",
    option: Some("password"),
};
pub const ERROR_PASSWORD_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(PASSWORD_ELEMENT_SIZE_JSON));

pub const OWNERS_NAME_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500 ",
    option: Some("ownres_name"),
};
pub const ERROR_OWNERS_NAME_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(OWNERS_NAME_ELEMENT_SIZE_JSON));

pub const NUMBER_ELEMENT_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 16 ",
    option: Some("number"),
};
pub const ERROR_NUMBER_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(NUMBER_ELEMENT_JSON));

pub const TYPE_CARD_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 50",
    option: Some("type_card"),
};
pub const ERROR_TYPE_CARD_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(TYPE_CARD_ELEMENT_SIZE_JSON));

pub const MONTH_CARD_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 2",
    option: Some("month_card"),
};
pub const ERROR_MONTH_CARD_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(MONTH_CARD_ELEMENT_SIZE_JSON));

pub const YEAR_CARD_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 4",
    option: Some("year_card"),
};
pub const ERROR_YEAR_CARD_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(YEAR_CARD_ELEMENT_SIZE_JSON));

pub const CCV_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 3",
    option: Some("ccv"),
};
pub const ERROR_CCV_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(CCV_ELEMENT_SIZE_JSON));

pub const FIRST_NAME_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 150",
    option: Some("first_name"),
};
pub const ERROR_FIRST_NAME_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(FIRST_NAME_ELEMENT_SIZE_JSON));

pub const SECOND_NAME_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 150",
    option: Some("second_name"),
};
pub const ERROR_SECOND_NAME_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(SECOND_NAME_ELEMENT_SIZE_JSON));

pub const LAST_NAME_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 200",
    option: Some("last_name"),
};
pub const ERROR_LAST_NAME_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(LAST_NAME_ELEMENT_SIZE_JSON));

pub const COMPANY_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500",
    option: Some("company"),
};
pub const ERROR_COMPANY_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(COMPANY_ELEMENT_SIZE_JSON));

pub const MAIL_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500",
    option: Some("mail"),
};
pub const ERROR_MAIL_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(MAIL_ELEMENT_SIZE_JSON));

pub const TEL_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 15 ",
    option: Some("tel"),
};
pub const ERROR_TEL_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(TEL_ELEMENT_SIZE_JSON));

pub const ADDRESS_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500 ",
    option: Some("address"),
};
pub const ERROR_ADDRESS_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(ADDRESS_ELEMENT_SIZE_JSON));

pub const CITY_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 100 ",
    option: Some("city"),
};
pub const ERROR_CITY_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(CITY_ELEMENT_SIZE_JSON));

pub const REGION_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 100",
    option: Some("region"),
};
pub const ERROR_REGION_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(REGION_ELEMENT_SIZE_JSON));

pub const INDEX_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 100 ",
    option: Some("index"),
};
pub const ERROR_INDEX_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(INDEX_ELEMENT_SIZE_JSON));

pub const COUNTRY_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 100",
    option: Some("contry"),
};
pub const ERROR_COUNTRY_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(COUNTRY_ELEMENT_SIZE_JSON));

pub const DESCRIPTION_ELEMENT_SIZE_JSON: ErrorResponse = ErrorResponse {
    cause: "Максимальна кількість символів 500 ",
    option: Some("description"),
};
pub const ERROR_DESCRIPTION_ELEMENT_SIZE: (Status, Json<ErrorResponse>) =
    (Status::BadRequest, Json(DESCRIPTION_ELEMENT_SIZE_JSON));

//min && max len login in element login
pub const LEN_LOGIN_IN_ELEMENT_LOGIN: LenText = LenText { min: 1, max: 500 };

//min && max len password in element login
pub const LEN_PASSWORD_IN_ELEMENT_LOGIN: LenText = LenText { min: 1, max: 500 };

//min && max len description in element login
pub const LEN_DESCRIPTION_IN_ELEMENT_LOGIN: LenText = LenText { min: 4, max: 800 };

//min && max len login in auth
pub const LEN_LOGIN_AUTH: LenText = LenText { min: 2, max: 200 };

//min && max len password in auth
pub const LEN_PASSWORD_AUTH: LenText = LenText { min: 8, max: 200 };

//min && max len first name
pub const LEN_FIRST_NAME: LenText = LenText { min: 2, max: 100 };

//min && max len last name
pub const LEN_LAST_NAME: LenText = LenText { min: 2, max: 150 };

pub const MAX_NAME_ELEMENT_SIZE: usize = 500;

pub const MAX_LOGIN_ELEMENT_SIZE: usize = 500;

pub const MAX_PASSWORD_ELEMENT_SIZE: usize = 500;

pub const MAX_OWNERS_NAME_ELEMENT_SIZE: usize = 500;

pub const NUMBER_ELEMENT_SIZE: usize = 16;

pub const MAX_TYPE_CARD_ELEMENT_SIZE: usize = 50;

pub const MAX_MONTH_CARD_ELEMENT_SIZE: usize = 2;

pub const YEAR_CARD_ELEMENT_SIZE: usize = 4;

pub const CCV_ELEMENT_SIZE: usize = 3;

pub const MAX_FIRST_NAME_ELEMENT_SIZE: usize = 150;

pub const MAX_SECOND_NAME_ELEMENT_SIZE: usize = 150;

pub const MAX_LAST_NAME_ELEMENT_SIZE: usize = 200;

pub const MAX_COMPANY_ELEMENT_SIZE: usize = 500;

pub const MAX_MAIL_ELEMENT_SIZE: usize = 500;

pub const MAX_TEL_ELEMENT_SIZE: usize = 15;

pub const MAX_ADDRESS_ELEMENT_SIZE: usize = 500;

pub const MAX_CITY_ELEMENT_SIZE: usize = 100;

pub const MAX_REGION_ELEMENT_SIZE: usize = 100;

pub const MAX_INDEX_ELEMENT_SIZE: usize = 100;

pub const MAX_COUNTRY_ELEMENT_SIZE: usize = 100;

pub const MAX_DESCRIPTION_ELEMENT_SIZE: usize = 500;
