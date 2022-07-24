use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::note_element_request::NoteElementRequest;
use crate::models::response::elements::error_elements::{ErrorElement, VecErrorsElementInModel};
use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use crate::routes::elements::post_elements::PostElementError;
use crate::{ErrorResponse, Status, UNKNOWN};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/note",
    format = "json",
    data = "<option_note_element_request>"
)]
pub async fn post_note(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
    option_note_element_request: Option<Json<NoteElementRequest>>,
) -> Result<
    Result<(Status, Json<Element>), (Status, Json<VecErrorsElementInModel>)>,
    (Status, Json<ErrorResponse>),
> {
    match check_note_request(option_note_element_request, auth.user_id) {
        Ok(PostNoteElementError::Ok(element)) => match database.post_element(&element).await {
            Ok(_) => Ok(Ok((Status::Ok, Json(element)))),
            Err(_) => Err(UNKNOWN),
        },
        Ok(PostNoteElementError::Unknown) => Err(UNKNOWN),
        Ok(PostNoteElementError::ErrorElements(errors_vec)) => Ok(Err((
            Status::BadRequest,
            Json(VecErrorsElementInModel { error: errors_vec }),
        ))),
        Err(_) => Err(WRONG_REQUEST),
    }
}

pub fn check_note_request(
    option_note_element: Option<Json<NoteElementRequest>>,
    id_user: String,
) -> Result<PostNoteElementError, ()> {
    match option_note_element {
        None => Err(()),
        Some(note_model) => {
            let element = from_note_model_to_element_model(note_model, id_user);
            match is_valid_element(&element) {
                PostElementError::Ok => Ok(PostNoteElementError::Ok(element)),
                PostElementError::Unknown => Ok(PostNoteElementError::Unknown),
                PostElementError::ErrorElements(vec_errors) => {
                    Ok(PostNoteElementError::ErrorElements(vec_errors))
                }
            }
        }
    }
}

pub fn from_note_model_to_element_model(
    note_model: Json<NoteElementRequest>,
    id_user: String,
) -> Element {
    Element {
        _id: ObjectId::new(),
        id_user,
        name: note_model.name.clone(),
        login: None,
        password: None,
        url: None,
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
        description: note_model.description.clone(),
        folder: note_model.folder.clone(),
        favorite: note_model.favourite,
    }
}

pub enum PostNoteElementError {
    Ok(Element),
    Unknown,
    ErrorElements(Vec<ErrorElement>),
}
