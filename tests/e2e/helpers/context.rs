use std::path::Path;

use assert_fs::NamedTempFile;

use crate::helpers::config_builder::ApplicationConfig;

pub(crate) struct E2ETestContext {
    config_file: NamedTempFile,
}

impl E2ETestContext {
    pub(crate) fn new() -> Self {
        E2ETestContext {
            config_file: NamedTempFile::new("application_config.yml").unwrap(),
        }
    }
}

impl E2ETestContext {
    pub(crate) fn create_config_file(&self, application_config: ApplicationConfig) {
        std::fs::write(
            self.config_file_path(),
            serde_yaml::to_string(&application_config).unwrap(),
        )
        .unwrap();
    }

    pub(crate) fn config_file_path(&self) -> &Path {
        self.config_file.path()
    }
}
