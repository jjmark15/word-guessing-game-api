use crate::domain::validated_guess::{ValidatedGuess, ValidatedLetter, Validity};

#[derive(derive_new::new)]
pub(crate) struct GuessValidator {}

impl GuessValidator {
    pub(crate) fn validate(
        &self,
        challenge_id: String,
        guess: String,
    ) -> Result<ValidatedGuess, ValidateGuessError> {
        let correct_word = self.get_correct_answer(challenge_id)?;

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

        Ok(ValidatedGuess::new(letters))
    }

    fn get_correct_answer(&self, challenge_id: String) -> Result<String, ChallengeNotFoundError> {
        if challenge_id != "challenge_id" {
            return Err(ChallengeNotFoundError(challenge_id));
        }
        Ok("guess".to_owned())
    }

    fn remove_first_instance_of_letter(letters: &mut Vec<char>, letter: &char) {
        let index = letters
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c == letter { Some(i) } else { None })
            .expect("list should contain letter");

        letters.remove(index);
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum ValidateGuessError {
    ChallengeNotFound(#[from] ChallengeNotFoundError),
}

#[derive(Debug, thiserror::Error)]
#[error("challenge with ID '{0}' not found")]
pub(crate) struct ChallengeNotFoundError(String);
