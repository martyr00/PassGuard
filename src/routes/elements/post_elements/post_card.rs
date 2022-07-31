use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::card_element_request::CardElementRequest;
use crate::models::response::elements::error_elements::{ErrorElement, VecErrorsElementInModel};
use crate::routes::user_routes::token::request_access_token::AuthorizedUser;
use crate::routes::elements::post_elements::PostElementError;
use crate::routes::validator_elements::is_valid_element;
use crate::{ErrorResponse, Status, UNKNOWN};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/card",
    format = "json",
    data = "<option_card_element_request>"
)]
pub async fn post_card(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
    option_card_element_request: Option<Json<CardElementRequest>>,
) -> Result<
    Result<(Status, Json<Element>), (Status, Json<VecErrorsElementInModel>)>,
    (Status, Json<ErrorResponse>),
> {
    match check_card_request(option_card_element_request, auth.user_id) {
        Ok(PostCardElementError::Ok(element)) => match database.post_element(&element).await {
            Ok(_) => Ok(Ok((Status::Ok, Json(element)))),
            Err(_) => Err(UNKNOWN),
        },
        Ok(PostCardElementError::ErrorElements(errors_vec)) => Ok(Err((
            Status::BadRequest,
            Json(VecErrorsElementInModel { error: errors_vec }),
        ))),
        Err(_) => Err(WRONG_REQUEST),
    }
}

pub fn check_card_request(
    option_login_element: Option<Json<CardElementRequest>>,
    id_user: String,
) -> Result<PostCardElementError, ()> {
    match option_login_element {
        None => Err(()),
        Some(card_model) => {
            let element = from_card_model_to_element_model(card_model, id_user);
            match is_valid_element(&element) {
                PostElementError::Ok => Ok(PostCardElementError::Ok(element)),
                PostElementError::ErrorElements(vec_errors) => {
                    Ok(PostCardElementError::ErrorElements(vec_errors))
                }
            }
        }
    }
}

pub fn from_card_model_to_element_model(
    card_model: Json<CardElementRequest>,
    id_user: String,
) -> Element {
    Element {
        _id: ObjectId::new(),
        id_user,
        name: card_model.name.clone(),
        login: None,
        password: None,
        url: None,
        owners_name: card_model.owners_name.clone(),
        number: card_model.number.clone(),
        type_card: card_model.type_card.clone(),
        month_card: card_model.month_card.clone(),
        year_card: card_model.year_card.clone(),
        ccv: card_model.ccv.clone(),
        first_name: card_model.first_name.clone(),
        second_name: card_model.second_name.clone(),
        last_name: card_model.last_name.clone(),
        company: None,
        mail: None,
        telephone: None,
        address_2: None,
        address_1: None,
        city: None,
        region: None,
        index: None,
        country: None,
        description: card_model.description.clone(),
        folder: card_model.folder.clone(),
        favorite: card_model.favourite,
    }
}

pub enum PostCardElementError {
    Ok(Element),
    ErrorElements(Vec<ErrorElement>),
}
