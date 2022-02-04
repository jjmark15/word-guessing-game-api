use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

use crate::application::ApplicationService;
use crate::domain;
use crate::domain::{ValidatedGuess, ValidatedLetter};

pub(crate) async fn validation_handler(
    Path(guess): Path<String>,
    Extension(application_service): Extension<Arc<ApplicationService>>,
) -> Response {
    let validated_guess: GuessValidationResponse = application_service.validate(guess).into();
    let body = Json(json!(validated_guess));

    (StatusCode::OK, body).into_response()
}

#[derive(Serialize, derive_new::new)]
struct GuessValidationResponse {
    letters: Vec<LetterValidation>,
}

#[derive(Serialize, derive_new::new)]
struct LetterValidation {
    letter: char,
    validation: Validity,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}

impl From<domain::ValidatedGuess> for GuessValidationResponse {
    fn from(from: ValidatedGuess) -> Self {
        GuessValidationResponse::new(from.letters().iter().map(LetterValidation::from).collect())
    }
}

impl From<&domain::ValidatedLetter> for LetterValidation {
    fn from(from: &ValidatedLetter) -> Self {
        LetterValidation::new(*from.letter(), from.validation().into())
    }
}

impl From<&domain::Validity> for Validity {
    fn from(from: &domain::Validity) -> Self {
        match from {
            domain::Validity::Correct => Validity::Correct,
            domain::Validity::Incorrect => Validity::Incorrect,
            domain::Validity::IncorrectPosition => Validity::IncorrectPosition,
        }
    }
}
