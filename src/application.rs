use crate::domain;

#[derive(derive_new::new)]
pub(crate) struct ApplicationService {
    guess_validator: domain::GuessValidator,
    answer_repository: domain::AnswerRepository,
}

impl ApplicationService {
    pub(crate) fn validate(
        &self,
        challenge_id: String,
        guess: String,
    ) -> Result<domain::ValidatedGuess, ValidateGuessError> {
        let answer = self.answer_repository.get(challenge_id.as_str())?;
        Ok(self.guess_validator.validate(answer, guess))
    }

    pub(crate) fn latest_challenge(&self) -> String {
        self.answer_repository.latest_id()
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) enum ValidateGuessError {
    ChallengeNotFound(#[from] domain::ChallengeNotFoundError),
}
