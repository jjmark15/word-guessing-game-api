#[derive(Debug, derive_new::new)]
pub(crate) struct ConfigValidator;

impl ConfigValidator {
    pub(crate) fn validate<C: Validated>(&self, config: &C) -> ValidationResult {
        config.validate()
    }
}

pub(crate) trait Validated {
    fn validate(&self) -> ValidationResult {
        Ok(())
    }
}

pub(crate) type ValidationResult = Result<(), ValidationError>;

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("invalid config: {message}")]
pub(crate) struct ValidationError {
    message: String,
}
