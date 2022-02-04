use serde::Deserialize;

use crate::api_error::ApiError;

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
    error: String,
}

impl From<ErrorResponse> for ApiError {
    fn from(from: ErrorResponse) -> Self {
        ApiError::new(from.error)
    }
}
