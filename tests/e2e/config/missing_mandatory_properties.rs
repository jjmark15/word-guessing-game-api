use speculoos::prelude::*;

use word_guessing_game_api::App;

use crate::helpers::config_builder::{ApplicationConfig, Builder, ServerConfig};
use crate::helpers::E2ETestContext;

#[tokio::test]
async fn fails_to_start_without_server_port() {
    let ctx = E2ETestContext::new();
    let application_config = ApplicationConfig::builder()
        .with_server(ServerConfig::builder().with_port(None))
        .build();
    ctx.create_config_file(application_config);

    let start_app_error = App::run(ctx.config_file_path())
        .await
        .err()
        .expect("app should fail to start");

    assert_that(&start_app_error.to_string())
        .is_equal_to(&"config is missing mandatory fields".to_string());
}
