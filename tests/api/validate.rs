use speculoos::prelude::*;

use api_test_client::http::header::CONTENT_TYPE;
use api_test_client::http::StatusCode;
use api_test_client::Validity;

use crate::helpers::assertions::HasValidationResults;
use crate::helpers::TestContext;

const CORRECT_GUESS: &str = "guess";
const GUESS_WITH_INCORRECT_LETTER: &str = "guesz";
const GUESS_WITH_INCORRECT_POSITION_LETTER: &str = "guesg";
const TOO_LONG_GUESS: &str = "guessy";
const TOO_SHORT_GUESS: &str = "gues";
const GUESS_WITH_WHITESPACE: &str = "gue s";
const GUESS_WITH_PUNCTUATION: &str = "gues!";
const GUESS_WITH_UPPERCASE: &str = "Guess";

#[tokio::test]
async fn validates_correct_guess() {
    let ctx = TestContext::new();

    let response = ctx.client().validate(CORRECT_GUESS).await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());

    let validation = response.value();

    assert_that(&validation.is_correct()).is_true();
    assert_that(&validation.letters().len()).is_equal_to(CORRECT_GUESS.len());
    assert_that(&validation.guess_string()).is_equal_to(CORRECT_GUESS.to_owned());
}

#[tokio::test]
async fn validates_guess_with_an_incorrect_letter() {
    let ctx = TestContext::new();

    let response = ctx.client().validate(GUESS_WITH_INCORRECT_LETTER).await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Incorrect,
    ]);
}

#[tokio::test]
async fn validates_guess_with_an_incorrect_position_letter() {
    let ctx = TestContext::new();

    let response = ctx
        .client()
        .validate(GUESS_WITH_INCORRECT_POSITION_LETTER)
        .await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::IncorrectPosition,
    ]);
}

#[tokio::test]
async fn rejects_guess_with_length_greater_than_5() {
    let ctx = TestContext::new();

    let response = ctx.client().validate(TOO_LONG_GUESS).await;

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

    let response = ctx.client().validate(TOO_SHORT_GUESS).await;

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

    let response = ctx.client().validate(GUESS_WITH_WHITESPACE).await;

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

    let response = ctx.client().validate(GUESS_WITH_PUNCTUATION).await;

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

    let response = ctx.client().validate(GUESS_WITH_UPPERCASE).await;

    assert_that(response.http_response_details().status_code())
        .is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.error().message()).is_equal_to(&"guess must be lowercase".to_string());
}
