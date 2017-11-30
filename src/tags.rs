extern crate serde_json;

use error::CliError;
use std::collections::HashMap;
use super::API;

type Tags = HashMap<String, String>;

impl API {
    // Return a list of tags
    pub fn tags(&self) -> Result<Tags, CliError> {
        let resp = self.get("tags/get")?;
        let tags: Tags = serde_json::from_value(resp)?;
        Ok(tags)
    }
}
