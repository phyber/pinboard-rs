/*
 * A library for working with the Pinboard API.
 */
#[cfg(test)] extern crate mockito;
#[cfg(test)] extern crate url;
extern crate reqwest;
#[cfg(test)] #[macro_use] extern crate serde_json;
#[cfg(not(test))] extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod error;
mod notes;
mod posts;
mod tags;
mod user;

// Use and re-export the PostAddBuilder
pub mod postaddbuilder;
#[cfg(test)] mod test_common;

use error::CliError;
use std::collections::HashMap;

#[cfg(test)]
static PINBOARD_API_URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
static PINBOARD_API_URL: &'static str = "https://api.pinboard.in/v1/";
static PINBOARD_RESPONSE_JSON: &'static str = "json";

/// Represents the information we need to keep around to use the Pinboard API.
pub struct API {
    client: reqwest::Client,
    token: String,
}

/// Represents additional arguments to pass to GET.
type GetArgs = HashMap<String, String>;

/*
 * Implementation of the PinboardAPI.
 *
 * Methods are named after the API endpoints.
 */
impl API {
    /// Returns an API client using the given token to authenticate.
    ///
    /// # Arguments
    ///
    /// * `token`: The API token of the Pinboard account to use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pinboard::API;
    ///
    /// let api = API::new("username:abcdef1234567890");
    /// ```
    pub fn new(token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.to_owned(),
        }
    }

    /// Returns a `reqwest::Url` suitable for requesting a Pinboard resource.
    ///
    /// # Arguments
    ///
    /// * `fragment`: A URL Fragment to append to the `PINBOARD_API_URL`, this
    ///   will form the full path that we want to `GET` from.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // In this case, `url` would be set to something along the lines of:
    /// // https://api.pinboard.in/v1/tags/get?auth_token=<token>&format=json
    /// let url = self.url("tags/get");
    /// ```
    fn url(&self, fragment: &str) -> reqwest::Url {
        // We unwrap here because we're using the URL defined above. It should
        // always be safe.
        let base = reqwest::Url::parse(PINBOARD_API_URL).unwrap();

        // We unwrap here since we're just being passed URL fragments from
        // other functions. This should be safe.
        let mut url = base.join(fragment).unwrap();

        // Set our query pairs.
        // We authenticate with our user token and we always want JSON replies.
        url.query_pairs_mut()
            .append_pair("auth_token", &self.token)
            .append_pair("format", PINBOARD_RESPONSE_JSON);

        // Finally return our Url
        url
    }

    /// Returns a `serde_json::Value` from the Pinboard API.
    ///
    /// # Arguments
    ///
    /// * `fragment`: A URL fragment representing the Pinboard resource we want
    ///   to request.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let json = self.get("tags/get")?;
    /// ```
    fn get(&self, fragment: &str) -> Result<serde_json::Value, CliError> {
        let url = self.url(fragment);
        let mut resp = self.client.get(url).send()?;
        let json = resp.json()?;
        Ok(json)
    }

    fn get_with_args(&self, fragment: &str, args: GetArgs) -> Result<serde_json::Value, CliError> {
        let mut url = self.url(fragment);

        // Loop over all arguments we got and build up the GET request.
        for (param, value) in &args {
            url.query_pairs_mut()
                .append_pair(&param, &value);
        }

        let mut resp = self.client.get(url).send()?;
        let json = resp.json()?;
        Ok(json)
    }
}
