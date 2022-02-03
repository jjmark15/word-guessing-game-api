use axum::http::StatusCode;

pub(crate) async fn status_handler() -> StatusCode {
    StatusCode::OK
}
