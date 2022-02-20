#[derive(Debug, thiserror::Error)]
pub(crate) enum ReadConfigError {
    #[error("could not read config from file")]
    ReadConfigFile,
    #[error("config is missing mandatory fields")]
    MissingMandatoryField,
}
