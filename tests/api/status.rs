use speculoos::prelude::*;

use api_test_client::http::StatusCode;

use crate::helpers::TestContext;

#[tokio::test]
async fn is_ok() {
    let ctx = TestContext::new();

    let response = ctx.client().status().await;

    assert_that(response.http_response_details().status_code()).is_equal_to(StatusCode::OK);
}
