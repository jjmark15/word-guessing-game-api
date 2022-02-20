use crate::helpers::config_builder::Builder;

#[derive(Clone, serde::Serialize)]
pub(crate) struct ChallengesConfig {
    answers: Option<Vec<String>>,
}

impl ChallengesConfig {
    pub(crate) fn builder() -> ChallengesConfigBuilder {
        ChallengesConfigBuilder::default()
    }
}

pub(crate) struct ChallengesConfigBuilder {
    answers: Option<Vec<String>>,
}

impl Default for ChallengesConfigBuilder {
    fn default() -> Self {
        ChallengesConfigBuilder {
            answers: Some(vec![]),
        }
    }
}

impl ChallengesConfigBuilder {
    pub(crate) fn with_answers(mut self, answers: impl Into<Option<Vec<String>>>) -> Self {
        self.answers = answers.into();
        self
    }

    fn build(self) -> ChallengesConfig {
        ChallengesConfig {
            answers: self.answers,
        }
    }
}

impl Builder for ChallengesConfigBuilder {
    type Target = ChallengesConfig;

    fn build(self) -> Self::Target {
        self.build()
    }
}
