use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::card_element_request::CardElementRequest;
use crate::{ErrorResponse, Status};
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/card",
    format = "json",
    data = "<option_card_element_request>"
)]
pub async fn post_card(
    database: &State<MongoDB>,
    option_card_element_request: Option<Json<CardElementRequest>>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {
}
