extern crate serde_json;

use error::CliError;
use super::{API, GetArgs};

// FIXME: Update time is an actual datetime, we should use `chrono` here.
// {"update_time":"2017-11-21T23:58:35Z"}
#[derive(Debug, Deserialize)]
pub struct PostUpdate {
    update_time: String,
}

impl API {
    pub fn post_update(&self) -> Result<PostUpdate, CliError> {
        let resp = self.get("posts/update")?;
        let update: PostUpdate = serde_json::from_value(resp)?;
        Ok(update)
    }
}
