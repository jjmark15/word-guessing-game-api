use speculoos::prelude::*;

use api_test_client::http::StatusCode;

use crate::helpers::TestContext;

#[tokio::test]
async fn validates_correct_guess() {
    let ctx = TestContext::new();
    let guess = "guess".to_owned();

    let response = ctx.client().validate(guess.clone()).await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);

    let validation = response.value();

    assert_that(&validation.is_correct()).is_true();
    assert_that(&validation.letters().len()).is_equal_to(guess.len());
    assert_that(&validation.guess_string()).is_equal_to(guess);
}
