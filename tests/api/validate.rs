use speculoos::prelude::*;

use api_test_client::http::header::CONTENT_TYPE;
use api_test_client::http::StatusCode;
use api_test_client::Validity;

use crate::helpers::assertions::HasValidationResults;
use crate::helpers::TestContext;

const CORRECT_GUESS: &str = "guess";
const GUESS_WITH_INCORRECT_LETTER: &str = "guesz";
const GUESS_WITH_INCORRECT_POSITION_LETTER: &str = "guesg";

#[tokio::test]
async fn validates_correct_guess() {
    let ctx = TestContext::new();

    let response = ctx.client().validate(CORRECT_GUESS.to_owned()).await;

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

    let response = ctx
        .client()
        .validate(GUESS_WITH_INCORRECT_LETTER.to_owned())
        .await;

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
        .validate(GUESS_WITH_INCORRECT_POSITION_LETTER.to_owned())
        .await;

    assert_that(response.value().letters()).has_validation_results(vec![
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::Correct,
        Validity::IncorrectPosition,
    ]);
}
