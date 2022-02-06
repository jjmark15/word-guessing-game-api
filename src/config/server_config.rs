#[derive(Debug, serde::Deserialize, derive_getters::Getters)]
pub(crate) struct ServerConfig {
    port: u16,
}
