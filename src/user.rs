extern crate serde_json;

use error::CliError;
use super::API;

#[derive(Debug, Deserialize)]
pub struct UserAPIToken {
    result: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSecret {
    result: String,
}

impl API {
    pub fn user_api_token(&self) -> Result<UserAPIToken, CliError> {
        let resp = self.get("user/api_token")?;
        let ret: UserAPIToken = serde_json::from_value(resp)?;
        Ok(ret)
    }

    pub fn user_secret(&self) -> Result<UserSecret, CliError> {
        let resp = self.get("user/secret")?;
        let ret: UserSecret = serde_json::from_value(resp)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use test_common;

    static USER_API_TOKEN_JSON: &'static str = "{\"result\":\"XOG86E7JIYMI\"}";
    static USER_SECRET_JSON: &'static str = "{\"result\":\"6493a84f72d86e7de130\"}";

    #[test]
    fn user_api_token() {
        let api = test_common::setup_api();
        let mock = test_common::setup_mock("user/api_token", USER_API_TOKEN_JSON);
        let ret = api.user_api_token();
        mock.assert();
        let user_api_token = ret.unwrap();

        assert_eq!(user_api_token.result, "XOG86E7JIYMI");
    }

    #[test]
    fn user_secret() {
        let api = test_common::setup_api();
        let mock = test_common::setup_mock("user/secret", USER_SECRET_JSON);
        let ret = api.user_secret();
        mock.assert();
        let user_secret = ret.unwrap();

        assert_eq!(user_secret.result, "6493a84f72d86e7de130");
    }
}
