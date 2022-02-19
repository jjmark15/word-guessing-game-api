use config::{ConfigError as ExternalConfigError, ConfigError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadConfigError {
    #[error("could not read config from file")]
    ReadConfigFile,
    #[error("config is missing mandatory fields")]
    MissingMandatoryField,
}

impl From<ExternalConfigError> for ReadConfigError {
    fn from(from: ExternalConfigError) -> Self {
        match from {
            ConfigError::Foreign { .. } => ReadConfigError::ReadConfigFile,
            ConfigError::Message(_) => ReadConfigError::MissingMandatoryField,
            _ => unimplemented!("{}", from),
        }
    }
}
