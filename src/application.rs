use crate::domain;

#[derive(derive_new::new)]
pub(crate) struct ApplicationService {
    guess_validator: domain::GuessValidator,
}

impl ApplicationService {
    pub(crate) fn validate(&self, guess: String) -> domain::ValidatedGuess {
        self.guess_validator.validate(guess)
    }
}
