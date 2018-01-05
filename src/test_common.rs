/*
 * Common testing functions, imported in per-file tests.
 */
use super::*;
use mockito;
use url::percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};

static AUTH_TOKEN: &'static str = "user:token";

pub fn setup_api() -> API {
    API::new(AUTH_TOKEN)
}

pub fn setup_mock(endpoint: &str, content: &str) -> mockito::Mock {
    let enc_token = utf8_percent_encode(
        AUTH_TOKEN,
        USERINFO_ENCODE_SET).to_string();

    let path = format!(
        "/{}?auth_token={}&format=json",
        endpoint,
        enc_token);

    mockito::mock("GET", path.as_str())
        .with_status(200)
        .with_header("content-type", "text/plain; charset=utf-8")
        .with_body(content)
        .create()
}
