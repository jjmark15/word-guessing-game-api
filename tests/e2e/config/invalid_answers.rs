use speculoos::prelude::*;

use word_guessing_game_api::App;

use crate::helpers::config_builder::{ApplicationConfig, Builder, ChallengesConfig};
use crate::helpers::E2ETestContext;

#[tokio::test]
async fn fails_to_start_without_challenge_answers() {
    let ctx = E2ETestContext::new();
    let application_config = ApplicationConfig::builder()
        .with_challenges(ChallengesConfig::builder().with_answers(None))
        .build();
    ctx.create_config_file(application_config);

    let start_app_error = App::run(ctx.config_file_path())
        .await
        .err()
        .expect("app should fail to start");

    assert_that(&start_app_error.to_string())
        .is_equal_to(&"config is missing mandatory fields".to_string());
}

#[tokio::test]
async fn fails_to_start_with_empty_challenge_answer_list() {
    let ctx = E2ETestContext::new();
    let application_config = ApplicationConfig::builder()
        .with_challenges(ChallengesConfig::builder().with_answers(vec![]))
        .build();
    ctx.create_config_file(application_config);

    let start_app_error = App::run(ctx.config_file_path())
        .await
        .err()
        .expect("app should fail to start");

    assert_that(&start_app_error.to_string())
        .is_equal_to(&"invalid config: challenge answer list is empty".to_string());
}
