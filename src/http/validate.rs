use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::application::ApplicationService;
use crate::http::response_body::GuessValidationResponse;

pub(crate) async fn validation_handler(
    Path(guess): Path<String>,
    Extension(application_service): Extension<Arc<ApplicationService>>,
) -> Response {
    let validated_guess: GuessValidationResponse = application_service.validate(guess).into();
    let body = Json(json!(validated_guess));

    (StatusCode::OK, body).into_response()
}
