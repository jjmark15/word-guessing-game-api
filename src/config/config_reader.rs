use std::path::Path;

use config::Config;

use crate::config::{ApplicationConfig, ReadConfigError};

#[derive(Debug, derive_new::new)]
pub(crate) struct ConfigReader {}

impl ConfigReader {
    pub(crate) fn read(&self, path: &Path) -> Result<ApplicationConfig, ReadConfigError> {
        let mut config = Config::default();
        config.merge(config::File::from(path))?;

        Ok(config.try_into::<ApplicationConfig>()?)
    }
}
