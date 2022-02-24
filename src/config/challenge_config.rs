use serde::Deserialize;

use crate::config::config_validator::{Validated, ValidationError, ValidationResult};

#[derive(Debug, Deserialize, derive_getters::Getters)]
pub(crate) struct ChallengeConfig {
    answers: Vec<String>,
}

impl Validated for ChallengeConfig {
    fn validate(&self) -> ValidationResult {
        if self.answers.is_empty() {
            return Err(ValidationError::new(
                "challenge answer list is empty".to_string(),
            ));
        }

        Ok(())
    }
}
