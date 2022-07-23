use crate::database::connect_to_db::MongoDB;
use crate::models::elements_model::Element;
use crate::models::request::elements::login_element_request::LoginElementRequest;
use crate::{ErrorResponse, Status};
use rocket::serde::json::Json;
use rocket::State;

#[post(
    "/element/login",
    format = "json",
    data = "<option_login_element_request>"
)]
pub async fn post_login(
    database: &State<MongoDB>,
    option_login_element_request: Option<Json<LoginElementRequest>>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {
}
