use reqwest::StatusCode;

use crate::endpoints::misc::ErrorResponse;

use axum::{
    Json,
    response::{IntoResponse, Response},
};

pub fn bad_request(message: &str) -> Response {
    return (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            message: message.to_owned(),
        }),
    )
        .into_response();
}
