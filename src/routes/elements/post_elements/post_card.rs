use rocket::serde::json::Json;
use rocket::State;
use crate::models::elements_model::Element;
use crate::{ErrorResponse, Status};
use crate::database::connect_to_db::MongoDB;

#[post("/element/card", format = "json", data = "<option_card_element_request>")]
pub async fn post_card(
    database: &State<MongoDB>,
    option_card_element_request: Option<Json<>>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {}