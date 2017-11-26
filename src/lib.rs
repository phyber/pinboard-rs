/*
 * A library for working with the Pinboard API.
 */
extern crate reqwest;
extern crate serde_json;

mod error;

use error::CliError;


// These statics are used when forming the API request URL.
static PINBOARD_API_URL: &'static str = "https://api.pinboard.in/v1/";
static PINBOARD_RESPONSE_JSON: &'static str = "json";

pub trait Tags {
    fn tags_get(&self) -> Result<serde_json::Value, CliError>;
}

pub struct API {
    client: reqwest::Client,
    token: String,
}

/*
 * Implementation of the PinboardAPI.
 *
 * Methods are named after the API endpoints.
 */
impl API {
    pub fn new(token: &str) -> API {
        API{
            client: reqwest::Client::new(),
            token: token.to_owned(),
        }
    }

    // Private method used by public facing API
    // Build a URL that we'll GET from.
    fn url(&self, fragment: &str) -> reqwest::Url {
        // We unwrap here because we're using the URL defined above. It should
        // always be safe.
        let base = reqwest::Url::parse(PINBOARD_API_URL).unwrap();

        // We unwrap here since we're just being passed URL fragments from
        // other functions. This should be safe.
        let mut url = base.join(&fragment).unwrap();

        // Set our query pairs.
        url.query_pairs_mut()
            .append_pair("auth_token", &self.token)
            .append_pair("format", PINBOARD_RESPONSE_JSON);

        // Finally returl our Url
        url
    }

    fn get(&self, fragment: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.url(&fragment);
        self.client.get(url).send()
    }
}

impl Tags for API {
    fn tags_get(&self) -> Result<serde_json::Value, CliError> {
        let mut resp = self.get("tags/get")?;
        // FIXME: Check it was actually a 200 before doing this.
        let body: serde_json::Value = resp.json()?;
        Ok(body)
    }
}
