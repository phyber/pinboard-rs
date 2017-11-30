extern crate serde_json;

use error::CliError;
use super::API;

impl API {
    // Return a list of tags
    pub fn tags(&self) -> Result<serde_json::Value, CliError> {
        let resp = self.get("tags/get")?;
        Ok(resp)
    }
}
