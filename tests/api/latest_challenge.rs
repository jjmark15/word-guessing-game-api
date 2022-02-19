use speculoos::prelude::*;

use api_test_client::http::header::CONTENT_TYPE;
use api_test_client::http::StatusCode;

use crate::helpers::TestContext;

#[tokio::test]
async fn gets_latest() {
    let ctx = TestContext::new();

    let response = ctx.client().latest_challenge().await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
    assert_that(&response.http_response_details().header_value(CONTENT_TYPE))
        .contains_value("application/json".to_owned());
    assert_that(&response.value()).is_equal_to("0".to_owned())
}
