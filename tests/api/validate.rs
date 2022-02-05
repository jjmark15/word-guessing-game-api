use speculoos::prelude::*;

use api_test_client::http::header::CONTENT_TYPE;
use api_test_client::http::StatusCode;
use api_test_client::Validity;

use crate::helpers::assertions::HasValidationResults;
use crate::helpers::TestContext;

#[tokio::test]
async fn validates_correct_guess() {
    let ctx = TestContext::new();
    let guess = "guess";
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, guess).await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());

    let validation = response.value();

    assert_that(validation.letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
    ]);
    assert_that(&validation.letters().len()).is_equal_to(guess.len());
    assert_that(&validation.guess_string()).is_equal_to(guess.to_owned());
}

#[tokio::test]
async fn validates_guess_with_a_correct_letter() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "gzzzz").await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
    ]);
}

#[tokio::test]
async fn validates_guess_with_an_incorrect_letter() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zzzzz").await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
    ]);
}

#[tokio::test]
async fn validates_guess_with_an_incorrect_position_letter() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zgzzz").await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Incorrect,
        Validity::IncorrectPosition,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
    ]);
}

#[tokio::test]
async fn unnecessary_correct_letter_in_incorrect_position_is_incorrect() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "ggzzz").await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
        Validity::Incorrect,
    ]);
}

#[tokio::test]
async fn rejects_guess_with_length_greater_than_5() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zzzzzz").await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message())
        .is_equal_to(&"guesses must have a length of 5 characters".to_string());
}

#[tokio::test]
async fn rejects_guess_with_length_less_than_5() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zzzz").await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message())
        .is_equal_to(&"guesses must have a length of 5 characters".to_string());
}

#[tokio::test]
async fn rejects_guess_with_whitespace() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zzz z").await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message())
        .is_equal_to(&"guesses must only contain letters".to_string());
}

#[tokio::test]
async fn rejects_guess_with_punctuation() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "zzzz!").await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message())
        .is_equal_to(&"guesses must only contain letters".to_string());
}

#[tokio::test]
async fn rejects_guess_with_uppercase_characters() {
    let ctx = TestContext::new();
    let challenge_id = ctx.client().latest_challenge().await.value();

    let response = ctx.client().validate(challenge_id, "Zzzzz").await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message()).is_equal_to(&"guess must be lowercase".to_string());
}

#[tokio::test]
async fn rejects_guess_for_non_existent_challenge() {
    let ctx = TestContext::new();

    let response = ctx
        .client()
        .validate("non-existent challenge", "zzzzz")
        .await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::NOT_FOUND);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message())
        .is_equal_to(&"challenge with ID 'non-existent challenge' not found".to_string());
}
