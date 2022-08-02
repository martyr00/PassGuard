use crate::constants::WRONG_REQUEST;
use crate::database::connect_to_db::MongoDB;
use crate::models::request::elements::elements_request::ElementRequest;
use crate::models::response::elements::post_elements_response::ResponseForPostElement;
use crate::routes::user_routes::token::request_access_token::AuthorizedUser;
use crate::{ErrorResponse, UNKNOWN};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/element", format = "json", data = "<option_element_request>")]
pub async fn post_element(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
    option_element_request: Option<Json<ElementRequest>>,
) -> Result<(Status, Json<ResponseForPostElement>), (Status, Json<ErrorResponse>)> {
    match option_element_request {
        Some(element) => {
            match database
                .post_element(
                    element.data_element.clone(),
                    element.data_preview.clone(),
                    auth.user_id.clone(),
                )
                .await
            {
                Ok(_) => {
                    let result = ResponseForPostElement {
                        user_id: auth.user_id.clone(),
                        element: element.data_element.clone(),
                        preview: element.data_preview.clone(),
                    };
                    Ok((Status::Ok, Json(result)))
                }
                Err(_) => Err(UNKNOWN),
            }
        }
        None => Err(WRONG_REQUEST),
    }
}
