use crate::database::connect_to_db::MongoDB;
use crate::models::response::elements::get_preview_response::ResponseGetPreview;
use crate::routes::user_routes::token::request_access_token::AuthorizedUser;
use crate::{ErrorResponse, UNKNOWN};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/element/preview")]
pub async fn get_preview_element(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
) -> Result<(Status, Json<ResponseGetPreview>), (Status, Json<ErrorResponse>)> {
    match database.get_previews(auth.user_id).await {
        Ok(vec_preview) => {
            if !vec_preview.is_empty() {
                Ok((Status::Ok, Json(ResponseGetPreview { data: vec_preview })))
            } else {
                Ok((
                    Status::NoContent,
                    Json(ResponseGetPreview { data: vec_preview }),
                ))
            }
        }
        Err(_) => Err(UNKNOWN),
    }
}
