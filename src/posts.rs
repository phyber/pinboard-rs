extern crate serde_json;

use error::CliError;
use super::{API, GetArgs};

#[derive(Debug, Deserialize)]
pub struct PostAdd {
    result_code: String,
}

// FIXME: Update time is an actual datetime, we should use `chrono` here.
// {"update_time":"2017-11-21T23:58:35Z"}
#[derive(Debug, Deserialize)]
pub struct PostUpdate {
    update_time: String,
}

impl API {
    pub fn post_add(&self, args: GetArgs) -> Result<PostAdd, CliError> {
        let resp = self.get_with_args("posts/add", args)?;
        let ret: PostAdd = serde_json::from_value(resp)?;
        Ok(ret)
    }

    pub fn post_update(&self) -> Result<PostUpdate, CliError> {
        let resp = self.get("posts/update")?;
        let update: PostUpdate = serde_json::from_value(resp)?;
        Ok(update)
    }
}
