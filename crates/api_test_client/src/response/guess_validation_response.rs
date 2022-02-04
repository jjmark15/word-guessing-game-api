use serde::Deserialize;

use crate::GuessValidation;

#[derive(Deserialize)]
pub(crate) struct GuessValidationResponse {
    letters: Vec<LetterValidation>,
}

#[derive(Deserialize)]
struct LetterValidation {
    letter: char,
    validation: Validity,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Validity {
    Correct,
    Incorrect,
    IncorrectPosition,
}

impl From<GuessValidationResponse> for GuessValidation {
    fn from(response: GuessValidationResponse) -> Self {
        GuessValidation::new(response.letters.into_iter().map(Into::into).collect())
    }
}

impl From<LetterValidation> for crate::guess_validation::LetterValidation {
    fn from(from: LetterValidation) -> Self {
        crate::guess_validation::LetterValidation::new(from.letter, from.validation.into())
    }
}

impl From<Validity> for crate::guess_validation::Validity {
    fn from(from: Validity) -> Self {
        match from {
            Validity::Correct => crate::guess_validation::Validity::Correct,
            Validity::Incorrect => crate::guess_validation::Validity::Incorrect,
            Validity::IncorrectPosition => crate::guess_validation::Validity::IncorrectPosition,
        }
    }
}
