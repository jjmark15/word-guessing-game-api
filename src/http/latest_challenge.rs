use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::http::response_body::LatestChallengeResponse;

pub(crate) async fn latest_challenge_handler() -> impl IntoResponse {
    let body = Json(json!(LatestChallengeResponse::new(
        "challenge_id".to_owned()
    )));

    (StatusCode::OK, body).into_response()
}
