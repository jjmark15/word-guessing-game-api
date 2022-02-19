use crate::config::challenge_config::ChallengeConfig;
use crate::config::server_config::ServerConfig;

#[derive(Debug, serde::Deserialize, derive_getters::Getters)]
pub(crate) struct ApplicationConfig {
    server: ServerConfig,
    challenges: ChallengeConfig,
}
