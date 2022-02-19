use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::http::response_body::LatestChallengeResponse;
use crate::ApplicationService;

pub(crate) async fn latest_challenge_handler(
    Extension(application_service): Extension<Arc<ApplicationService>>,
) -> impl IntoResponse {
    let challenge_id = application_service.latest_challenge();
    let body = Json(json!(LatestChallengeResponse::new(challenge_id)));

    (StatusCode::OK, body).into_response()
}
