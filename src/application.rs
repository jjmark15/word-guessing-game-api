use crate::domain;

#[derive(derive_new::new)]
pub(crate) struct ApplicationService {
    guess_validator: domain::GuessValidator,
}

impl ApplicationService {
    pub(crate) fn validate(
        &self,
        challenge_id: String,
        guess: String,
    ) -> Result<domain::ValidatedGuess, domain::ValidateGuessError> {
        self.guess_validator.validate(challenge_id, guess)
    }
}
