extern crate serde_json;

use error::CliError;
use std::collections::HashMap;
use super::API;

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
}
