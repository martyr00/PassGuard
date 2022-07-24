use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::login_element_request::LoginElementRequest;
use crate::models::response::elements::error_elements::{ErrorElement, VecErrorsElementInModel};
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::routes::elements::post_elements::PostElementError;
use crate::{ErrorResponse, Status, UNKNOWN};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/login",
    format = "json",
    data = "<option_login_element_request>"
)]
pub async fn post_login(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
    option_login_element_request: Option<Json<LoginElementRequest>>,
) -> Result<
    Result<(Status, Json<Element>), (Status, Json<VecErrorsElementInModel>)>,
    (Status, Json<ErrorResponse>),
> {
    match check_login_request(option_login_element_request, auth.user_id) {
        Ok(PostLoginElementError::Ok(element)) => match database.post_element(&element).await {
            Ok(_) => Ok(Ok((Status::Ok, Json(element)))),
            Err(_) => Err(UNKNOWN),
        },
        Ok(PostLoginElementError::Unknown) => Err(UNKNOWN),
        Ok(PostLoginElementError::ErrorElements(errors_vec)) => Ok(Err((
            Status::BadRequest,
            Json(VecErrorsElementInModel { error: errors_vec }),
        ))),
        Err(_) => Err(WRONG_REQUEST),
    }
}

pub fn check_login_request(
    option_login_element: Option<Json<LoginElementRequest>>,
    id_user: String,
) -> Result<PostLoginElementError, ()> {
    match option_login_element {
        None => Err(()),
        Some(login_model) => {
            let element = from_login_model_to_element_model(login_model, id_user);
            match is_valid_element(&element) {
                PostElementError::Ok => Ok(PostLoginElementError::Ok(element)),
                PostElementError::Unknown => Ok(PostLoginElementError::Unknown),
                PostElementError::ErrorElements(vec_errors) => {
                    Ok(PostLoginElementError::ErrorElements(vec_errors))
                }
            }
        }
    }
}

pub fn from_login_model_to_element_model(
    login_model: Json<LoginElementRequest>,
    id_user: String,
) -> Element {
    Element {
        _id: ObjectId::new(),
        id_user,
        name: login_model.name.clone(),
        login: login_model.login.clone(),
        password: login_model.password.clone(),
        url: login_model.url.clone(),
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
        description: login_model.description.clone(),
        folder: login_model.folder.clone(),
        favorite: login_model.favourite,
    }
}

pub enum PostLoginElementError {
    Ok(Element),
    Unknown,
    ErrorElements(Vec<ErrorElement>),
}
