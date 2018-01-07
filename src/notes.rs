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
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Notes {
    pub count: i64,
    pub notes: Vec<NotesNote>,
}

// Pinboard API docs don't mention created_at and updated_at being returned
// here as they aren't present in the XML reply.
#[derive(Debug, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub length: i64,
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub text: String,
}

impl API {
    // Return a list of notes.
    pub fn notes(&self) -> Result<Notes, CliError> {
        let resp = self.get("notes/list")?;
        let notes: Notes = serde_json::from_value(resp)?;
        Ok(notes)
    }

    // Get a specific note
    pub fn note(&self, id: &str) -> Result<Note, CliError> {
        let fragment = format!("notes/{}", id);
        let resp = self.get(&fragment)?;
        let note: Note = serde_json::from_value(resp)?;
        Ok(note)
    }
}

#[cfg(test)]
mod tests {
    use test_common;

    #[test]
    fn notes() {
        let json = json!({
            "count":1,
            "notes": [
                {
                    "id": "8e5d6964bb810e0050b0",
                    "hash": "0c9c30f60cadabd31415",
                    "title": "StarCraft beta coming this week!",
                    "length": "153",
                    "created_at": "2010-02-11 03:46:56",
                    "updated_at": "2010-02-11 03:47:47",
                },
            ],
        }).to_string();

        let api = test_common::setup_api();
        let mock = test_common::setup_mock("notes/list", &json);

        // Notes List call
        let ret = api.notes();
        mock.assert();
        let notes = ret.unwrap();

        assert_eq!(notes.count, 1);
        assert_eq!(notes.notes.len(), 1);
        assert_eq!(notes.notes[0].created_at, "2010-02-11 03:46:56");
        assert_eq!(notes.notes[0].hash, "0c9c30f60cadabd31415");
        assert_eq!(notes.notes[0].id, "8e5d6964bb810e0050b0");
        assert_eq!(notes.notes[0].length, "153");
        assert_eq!(notes.notes[0].title, "StarCraft beta coming this week!");
        assert_eq!(notes.notes[0].updated_at, "2010-02-11 03:47:47");
    }

    #[test]
    fn note() {
        let json = json!({
            "id": "8e5d6964bb810e0050b0",
            "title": "StarCraft beta coming this week!",
            "created_at": "2010-02-11 03:46:56",
            "updated_at": "2010-02-11 03:47:47",
            "length": 153,
            "text": "<![CDATA[It was clear that readers showing up for our live blog of the Activision Blizzard earnings call were interested in one thing: when the closed beta for StarCraft 2 was set to begin. It took a while to get there, but we were provided a solid answer. The beta will begin before the end of the month, with the game itself set for release in the middle of 2010.]]>",
            "hash": "0c9c30f60cadabd31415",
        }).to_string();

        let api = test_common::setup_api();
        let mock = test_common::setup_mock("notes/8e5d6964bb810e0050b0", &json);

        let ret = api.note("8e5d6964bb810e0050b0");
        mock.assert();
        let note = ret.unwrap();

        assert_eq!(note.created_at, "2010-02-11 03:46:56");
        assert_eq!(note.hash, "0c9c30f60cadabd31415");
        assert_eq!(note.id, "8e5d6964bb810e0050b0");
        assert_eq!(note.length, 153);
        assert_eq!(note.text, "<![CDATA[It was clear that readers showing up for our live blog of the Activision Blizzard earnings call were interested in one thing: when the closed beta for StarCraft 2 was set to begin. It took a while to get there, but we were provided a solid answer. The beta will begin before the end of the month, with the game itself set for release in the middle of 2010.]]>");
        assert_eq!(note.title, "StarCraft beta coming this week!");
        assert_eq!(note.updated_at, "2010-02-11 03:47:47");
    }
}
