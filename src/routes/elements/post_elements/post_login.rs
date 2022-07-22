use rocket::serde::json::Json;
use rocket::State;
use crate::models::elements_model::Element;
use crate::{ErrorResponse, Status};
use crate::database::connect_to_db::MongoDB;

#[post("/element/login", format = "json", data = "<option_login_element_request>")]
pub async fn post_login(
    database: &State<MongoDB>,
    option_login_element_request: Option<Json<>>,
) -> Result<(Status, Json<Element>), (Status, Json<ErrorResponse>)> {}