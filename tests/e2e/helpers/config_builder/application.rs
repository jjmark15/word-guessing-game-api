use crate::helpers::config_builder::challenges::{ChallengesConfig, ChallengesConfigBuilder};
use crate::helpers::config_builder::server::{ServerConfig, ServerConfigBuilder};
use crate::helpers::config_builder::{Builder, OptionalBuilder};

#[derive(serde::Serialize)]
pub(crate) struct ApplicationConfig {
    server: Option<ServerConfig>,
    challenges: Option<ChallengesConfig>,
}

pub(crate) struct ApplicationConfigBuilder {
    server: Option<ServerConfig>,
    challenges: Option<ChallengesConfig>,
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        ApplicationConfigBuilder {
            server: Some(Builder::build(ServerConfigBuilder::default())),
            challenges: Some(Builder::build(ChallengesConfigBuilder::default())),
        }
    }
}

impl ApplicationConfig {
    pub(crate) fn builder() -> ApplicationConfigBuilder {
        ApplicationConfigBuilder::default()
    }
}

impl ApplicationConfigBuilder {
    pub(crate) fn with_server(
        mut self,
        server: impl OptionalBuilder<Builder = ServerConfigBuilder>,
    ) -> Self {
        self.server = server.build();
        self
    }

    pub(crate) fn with_challenges(
        mut self,
        challenges: impl OptionalBuilder<Builder = ChallengesConfigBuilder>,
    ) -> Self {
        self.challenges = challenges.build();
        self
    }

    fn build(self) -> ApplicationConfig {
        ApplicationConfig {
            server: self.server,
            challenges: self.challenges,
        }
    }
}

impl Builder for ApplicationConfigBuilder {
    type Target = ApplicationConfig;

    fn build(self) -> Self::Target {
        self.build()
    }
}
