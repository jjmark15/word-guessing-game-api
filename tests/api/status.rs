use speculoos::prelude::*;

use api_test_client::http::StatusCode;

use crate::helpers::TestContext;

#[tokio::test]
async fn is_ok() {
    let ctx = TestContext::new();

    let status = ctx.client().status().await;

    assert_that(&status).is_equal_to(StatusCode::OK);
}
