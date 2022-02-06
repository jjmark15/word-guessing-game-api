use std::path::PathBuf;

use assert_fs::NamedTempFile;
use once_cell::unsync::OnceCell;

use crate::helpers::config_builder::ApplicationConfig;

#[derive(derive_new::new)]
pub(crate) struct E2ETestContext {
    #[new(default)]
    config_file: OnceCell<NamedTempFile>,
}

impl E2ETestContext {
    pub(crate) fn create_config_file(&mut self, application_config: ApplicationConfig) {
        std::fs::write(
            self.config_file_path(),
            serde_yaml::to_string(&application_config).unwrap(),
        )
        .unwrap();
    }

    pub(crate) fn config_file_path(&self) -> PathBuf {
        self.config_file
            .get_or_init(|| NamedTempFile::new("application_config.yml").unwrap())
            .to_path_buf()
    }
}
