use config::{ConfigError as ExternalConfigError, ConfigError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadConfigError {
    #[error("config contains invalid content")]
    InvalidConfigContent,
    #[error("could not read config from file")]
    ReadConfigFile,
}

impl From<ExternalConfigError> for ReadConfigError {
    fn from(from: ExternalConfigError) -> Self {
        match from {
            ConfigError::Type { .. } => ReadConfigError::InvalidConfigContent,
            ConfigError::Foreign { .. } => ReadConfigError::ReadConfigFile,
            _ => unimplemented!(),
        }
    }
}
