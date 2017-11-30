/*
 * A library for working with the Pinboard API.
 */
extern crate reqwest;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod error;
mod tags;

use error::CliError;

// These statics are used when forming the API request URL.
static PINBOARD_API_URL: &'static str = "https://api.pinboard.in/v1/";
static PINBOARD_RESPONSE_JSON: &'static str = "json";

pub struct API {
    client: reqwest::Client,
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct NotesNote {
    pub id: String,
    pub title: String,
    pub length: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Notes {
    pub count: i64,
    pub notes: Vec<NotesNote>,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub length: i64,
    pub created_at: String,
    pub updated_at: String,
    pub text: String,
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
        let mut url = base.join(fragment).unwrap();

        // Set our query pairs.
        url.query_pairs_mut()
            .append_pair("auth_token", &self.token)
            .append_pair("format", PINBOARD_RESPONSE_JSON);

        // Finally returl our Url
        url
    }

    fn get(&self, fragment: &str) -> Result<serde_json::Value, CliError> {
        let url = self.url(fragment);
        let mut resp = self.client.get(url).send()?;
        let json = resp.json()?;
        Ok(json)
    }

    // Return a list of notes.
    //pub fn notes(&self) -> Result<serde_json::Value, CliError> {
    pub fn notes(&self) -> Result<Notes, CliError> {
        let resp = self.get("notes/list")?;
        let notes: Notes = serde_json::from_value(resp)?;
        Ok(notes)
    }

    // Get a specific note
    pub fn note(&self, id: String) -> Result<Note, CliError> {
        let fragment = format!("notes/{}", id);
        let resp = self.get(&fragment)?;
        let note: Note = serde_json::from_value(resp)?;
        Ok(note)
    }
}
