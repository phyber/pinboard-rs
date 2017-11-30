/*
 * A library for working with the Pinboard API.
 */
extern crate serde_json;

use error::CliError;
use super::API;

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

impl API {
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
