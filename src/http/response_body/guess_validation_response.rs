use serde::Serialize;

use crate::domain;

#[derive(Serialize, derive_new::new)]
pub(crate) struct GuessValidationResponse {
    letters: Vec<LetterValidation>,
}

#[derive(Serialize, derive_new::new)]
pub(crate) struct LetterValidation {
    letter: char,
    validation: Validity,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}

impl From<domain::ValidatedGuess> for GuessValidationResponse {
    fn from(from: domain::ValidatedGuess) -> Self {
        GuessValidationResponse::new(from.letters().iter().map(LetterValidation::from).collect())
    }
}

impl From<&domain::ValidatedLetter> for LetterValidation {
    fn from(from: &domain::ValidatedLetter) -> Self {
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
