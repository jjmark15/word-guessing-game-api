use std::path::Path;

use config::Config;

use crate::config::{ApplicationConfig, ReadConfigError};

#[derive(Debug, derive_new::new)]
pub(crate) struct ConfigReader {}

impl ConfigReader {
    pub(crate) fn read(&self, path: &Path) -> Result<ApplicationConfig, ReadConfigError> {
        let settings = Config::builder()
            .add_source(config::File::from(path))
            .build()
            .map_err(ReadConfigError::from)?;

        settings
            .try_deserialize::<ApplicationConfig>()
            .map_err(ReadConfigError::from)
    }
}
