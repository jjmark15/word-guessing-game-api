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
) -> Result<Response, ValidateGuessError> {
    reject_invalid_guess_input(&guess)?;

    let validated_guess: GuessValidationResponse = application_service.validate(guess).into();
    let body = Json(json!(validated_guess));

    Ok((StatusCode::OK, body).into_response())
}

fn reject_invalid_guess_input(guess: &str) -> Result<(), ValidateGuessError> {
    if guess.len() != 5 {
        return Err(ValidateGuessError::IncorrectLength);
    }

    for c in guess.chars() {
        if c.is_uppercase() {
            return Err(ValidateGuessError::Uppercase);
        } else if !c.is_alphabetic() {
            return Err(ValidateGuessError::NotLetters);
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ValidateGuessError {
    #[error("guesses must have a length of 5 characters")]
    IncorrectLength,
    #[error("guesses must only contain letters")]
    NotLetters,
    #[error("guess must be lowercase")]
    Uppercase,
}

impl IntoResponse for ValidateGuessError {
    fn into_response(self) -> Response {
        let status = match self {
            ValidateGuessError::IncorrectLength
            | ValidateGuessError::NotLetters
            | ValidateGuessError::Uppercase => StatusCode::NOT_ACCEPTABLE,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}
