extern crate serde_json;

use error::CliError;
use std::collections::HashMap;
use super::{API, GetArgs};

type Tags = HashMap<String, i64>;

impl API {
    // Return a list of tags
    pub fn tags(&self) -> Result<Tags, CliError> {
        let resp = self.get("tags/get")?;

        // Temporary HashMap of String,String.
        let tmp: HashMap<String, String> = serde_json::from_value(resp)?;

        // Our new HashMap of String,i64
        let mut tags = Tags::new();

        // Loop over the tmp HashMap, transforming the counts into proper i64s.
        // This should be possible during deserialization, but the serde runes
        // for this are currently unknown.
        for (name, count_str) in &tmp {
            let count = count_str.parse::<i64>()?;
            tags.insert(name.to_owned(), count);
        }

        Ok(tags)
    }

    pub fn tag_delete(&self, tag: &str) -> Result<(), CliError> {
        let mut args = GetArgs::new();
        args.insert("tag".to_string(), tag.to_string());

        // FIXME: Use the resp here.
        let _resp = self.get_with_args("tags/delete", args)?;
        Ok(())
    }

    pub fn tag_rename(&self, old: &str, new: &str) -> Result<(), CliError> {
        let mut args = GetArgs::new();
        args.insert("old".to_string(), old.to_string());
        args.insert("new".to_string(), new.to_string());

        // FIXME: Use the resp here.
        let _resp = self.get_with_args("tags/rename", args)?;
        Ok(())
    }
}
