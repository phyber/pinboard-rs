extern crate reqwest;
extern crate serde_json;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Api(reqwest::Error),
    Json(serde_json::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Api(ref err) => write!(f, "API error: {}", err),
            CliError::Json(ref err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Api(ref err) => err.description(),
            CliError::Json(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Api(ref err) => Some(err),
            CliError::Json(ref err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
        CliError::Api(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> CliError {
        CliError::Json(err)
    }
}
