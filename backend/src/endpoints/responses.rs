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

pub fn server_error(message: &str) -> Response {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: message.to_owned(),
        }),
    )
        .into_response();
}

pub fn confilict(message: &str) -> Response {
    return (
        StatusCode::CONFLICT,
        Json(ErrorResponse {
            message: message.to_owned(),
        }),
    )
        .into_response();
}
