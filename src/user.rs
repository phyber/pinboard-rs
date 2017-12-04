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
