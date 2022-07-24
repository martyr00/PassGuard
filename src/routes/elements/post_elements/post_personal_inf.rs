use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::personal_element_request::PersonalElementRequest;
use crate::models::response::elements::error_elements::{ErrorElement, VecErrorsElementInModel};
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::routes::elements::post_elements::PostElementError;
use crate::{ErrorResponse, Status, UNKNOWN};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/personal",
    format = "json",
    data = "<option_personal_inf_element_request>"
)]
pub async fn post_personal(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
    option_personal_inf_element_request: Option<Json<PersonalElementRequest>>,
) -> Result<
    Result<(Status, Json<Element>), (Status, Json<VecErrorsElementInModel>)>,
    (Status, Json<ErrorResponse>),
> {
    match check_personal_inf_request(option_personal_inf_element_request, auth.user_id) {
        Ok(PostPersonalInfElementError::Ok(element)) => match database.post_element(&element).await
        {
            Ok(_) => Ok(Ok((Status::Ok, Json(element)))),
            Err(_) => Err(UNKNOWN),
        },
        Ok(PostPersonalInfElementError::Unknown) => Err(UNKNOWN),
        Ok(PostPersonalInfElementError::ErrorElements(errors_vec)) => Ok(Err((
            Status::BadRequest,
            Json(VecErrorsElementInModel { error: errors_vec }),
        ))),
        Err(_) => Err(WRONG_REQUEST),
    }
}

pub fn check_personal_inf_request(
    option_personal_inf_element: Option<Json<PersonalElementRequest>>,
    id_user: String,
) -> Result<PostPersonalInfElementError, ()> {
    match option_personal_inf_element {
        None => Err(()),
        Some(personal_inf_model) => {
            let element = from_personal_inf_model_to_element_model(personal_inf_model, id_user);
            match is_valid_element(&element) {
                PostElementError::Ok => Ok(PostPersonalInfElementError::Ok(element)),
                PostElementError::Unknown => Ok(PostPersonalInfElementError::Unknown),
                PostElementError::ErrorElements(vec_errors) => {
                    Ok(PostPersonalInfElementError::ErrorElements(vec_errors))
                }
            }
        }
    }
}

pub fn from_personal_inf_model_to_element_model(
    personal_inf_model: Json<PersonalElementRequest>,
    id_user: String,
) -> Element {
    Element {
        _id: ObjectId::new(),
        id_user,
        name: personal_inf_model.name.clone(),
        login: None,
        password: None,
        url: None,
        owners_name: None,
        number: None,
        type_card: None,
        month_card: None,
        year_card: None,
        ccv: None,
        first_name: personal_inf_model.first_name.clone(),
        second_name: personal_inf_model.second_name.clone(),
        last_name: personal_inf_model.last_name.clone(),
        company: personal_inf_model.company.clone(),
        mail: personal_inf_model.mail.clone(),
        telephone: personal_inf_model.telephone.clone(),
        address_2: personal_inf_model.address_2.clone(),
        address_1: personal_inf_model.address_1.clone(),
        city: personal_inf_model.city.clone(),
        region: personal_inf_model.region.clone(),
        index: personal_inf_model.index.clone(),
        country: personal_inf_model.country.clone(),
        description: personal_inf_model.description.clone(),
        folder: personal_inf_model.folder.clone(),
        favorite: personal_inf_model.favourite,
    }
}

pub enum PostPersonalInfElementError {
    Ok(Element),
    Unknown,
    ErrorElements(Vec<ErrorElement>),
}
