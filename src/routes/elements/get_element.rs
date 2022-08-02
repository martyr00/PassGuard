use crate::database::connect_to_db::MongoDB;
use crate::models::element_model::Element;
use crate::routes::user_routes::token::request_access_token::AuthorizedUser;
use crate::{ErrorResponse, UNKNOWN};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/element/<element_id>")]
pub async fn get_element(
    _auth: AuthorizedUser,
    database: &State<MongoDB>,
    element_id: String,
) -> Result<(Status, Json<Option<Element>>), (Status, Json<ErrorResponse>)> {
    match database.get_element(element_id).await {
        Ok(option_element) => match option_element {
            None => Ok((Status::NoContent, Json(None))),
            Some(element) => Ok((Status::Ok, Json(Some(element)))),
        },
        Err(_) => Err(UNKNOWN),
    }
}
