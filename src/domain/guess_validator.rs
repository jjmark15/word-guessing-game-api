use crate::domain::validated_guess::{ValidatedGuess, ValidatedLetter, Validity};

#[derive(derive_new::new)]
pub(crate) struct GuessValidator {}

impl GuessValidator {
    pub(crate) fn validate(&self, guess: String) -> ValidatedGuess {
        let correct_word = "guess";

        let letters = correct_word
            .chars()
            .zip(guess.chars())
            .map(|(correct_char, char)| {
                if correct_char == char {
                    ValidatedLetter::new(char, Validity::Correct)
                } else if correct_word.contains(char) {
                    ValidatedLetter::new(char, Validity::IncorrectPosition)
                } else {
                    ValidatedLetter::new(char, Validity::Incorrect)
                }
            })
            .collect();

        ValidatedGuess::new(letters)
    }
}
