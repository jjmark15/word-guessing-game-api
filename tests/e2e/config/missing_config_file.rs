use speculoos::prelude::*;

use word_guessing_game_api::App;

use crate::helpers::E2ETestContext;

#[tokio::test]
async fn fails_to_start_if_config_file_not_found() {
    let ctx = E2ETestContext::new();

    let start_app_error = App::run(ctx.config_file_path().as_path())
        .await
        .err()
        .expect("app should fail to start");

    assert_that(&start_app_error.to_string())
        .is_equal_to(&"could not read config from file".to_string());
}
