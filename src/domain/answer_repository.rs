#[derive(derive_new::new)]
pub(crate) struct AnswerRepository {
    answers: Vec<String>,
}

impl AnswerRepository {
    pub(crate) fn get(&self, challenge_id: &str) -> Result<String, ChallengeNotFoundError> {
        self.answers
            .get(self.answer_id_to_index(challenge_id)?)
            .cloned()
            .ok_or_else(|| ChallengeNotFoundError(challenge_id.to_owned()))
    }

    fn answer_id_to_index(&self, answer_id: &str) -> Result<usize, ChallengeNotFoundError> {
        answer_id
            .parse()
            .map_err(|_e| ChallengeNotFoundError(answer_id.to_owned()))
    }

    pub(crate) fn latest_id(&self) -> String {
        (self.answers.len() - 1).min(0).to_string()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("challenge with ID '{0}' not found")]
pub(crate) struct ChallengeNotFoundError(String);
