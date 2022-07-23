use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::personal_element_request::PersonalElementRequest;
use crate::{ErrorResponse, Status};
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/personal",
    format = "json",
    data = "<option_personal_inf_element_request>"
)]
pub async fn post_personal(
    database: &State<MongoDB>,
    option_personal_inf_element_request: Option<Json<PersonalElementRequest>>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {
}
