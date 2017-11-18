/*
 * A library for working with the Pinboard API.
 */
extern crate reqwest;
use reqwest::{Client, Error, Response, Url};

// URL where the API lives.
static PINBOARD_API_URL: &'static str = "https://api.pinboard.in/v1/";

pub trait Tags {
    fn tags_get(&self) -> Result<Response, Error>;
}

pub struct API {
    client: Client,
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
            client: Client::new(),
            token: token.to_owned(),
        }
    }

    // Private method used by public facing API
    // Build a URL that we'll GET from.
    fn url(&self, fragment: &str) -> Url {
        // We unwrap here because we're using the URL defined above. It should
        // always be safe.
        let base = Url::parse(PINBOARD_API_URL).unwrap();

        // We unwrap here since we're just being passed URL fragments from
        // other functions. This should be safe.
        let mut url = base.join(&fragment).unwrap();

        // Set our query pairs.
        url.query_pairs_mut()
            .append_pair("auth_token", &self.token)
            .append_pair("format", &String::from("json"));

        // Finally returl our Url
        url
    }

    fn get(&self, fragment: &str) -> Result<Response, Error> {
        let url = self.url(&fragment);
        self.client.get(url).send()
    }
}

impl Tags for API {
    fn tags_get(&self) -> Result<Response, Error> {
        self.get("tags/get")
    }
}
