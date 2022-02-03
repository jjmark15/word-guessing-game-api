use crate::domain::validated_guess::{ValidatedGuess, ValidatedLetter, Validity};

#[derive(derive_new::new)]
pub(crate) struct GuessValidator {}

impl GuessValidator {
    pub(crate) fn validate(&self, guess: String) -> ValidatedGuess {
        let letters = guess
            .chars()
            .into_iter()
            .map(|c| ValidatedLetter::new(c, Validity::Correct))
            .collect();
        ValidatedGuess::new(letters)
    }
}
