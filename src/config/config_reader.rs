use std::path::Path;

use config::Config;

use crate::config::{ApplicationConfig, ReadConfigError};

#[derive(Debug, derive_new::new)]
pub(crate) struct ConfigReader {}

impl ConfigReader {
    pub(crate) fn read(&self, path: &Path) -> Result<ApplicationConfig, ReadConfigError> {
        Config::builder()
            .add_source(config::File::from(path))
            .build()
            .map_err(|_e| ReadConfigError::ReadConfigFile)?
            .try_deserialize::<ApplicationConfig>()
            .map_err(|_e| ReadConfigError::MissingMandatoryField)
    }
}
