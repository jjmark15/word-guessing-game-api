use serde::Deserialize;

#[derive(Debug, Deserialize, derive_getters::Getters)]
pub(crate) struct ChallengeConfig {
    answers: Vec<String>,
}
