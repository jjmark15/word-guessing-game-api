use crate::domain::validated_guess::{ValidatedGuess, ValidatedLetter, Validity};

#[derive(derive_new::new)]
pub(crate) struct GuessValidator {}

impl GuessValidator {
    pub(crate) fn validate(&self, guess: String) -> ValidatedGuess {
        let correct_word = "guess";

        let mut unused_letters: Vec<char> = correct_word.chars().collect();

        let letters = correct_word
            .chars()
            .zip(guess.chars())
            .map(|(correct_char, char)| {
                if !unused_letters.contains(&char) {
                    ValidatedLetter::new(char, Validity::Incorrect)
                } else if correct_char == char {
                    Self::remove_first_instance_of_letter(&mut unused_letters, &char);
                    ValidatedLetter::new(char, Validity::Correct)
                } else {
                    ValidatedLetter::new(char, Validity::IncorrectPosition)
                }
            })
            .collect();

        ValidatedGuess::new(letters)
    }

    fn remove_first_instance_of_letter(letters: &mut Vec<char>, letter: &char) {
        let index = letters
            .iter()
            .enumerate()
            .find(|(_i, c)| c == &letter)
            .map(|(i, _)| i)
            .expect("list should contain letter");

        letters.remove(index);
    }
}
