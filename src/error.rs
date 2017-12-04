extern crate reqwest;
extern crate serde_json;

use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum CliError {
    Api(reqwest::Error),
    Json(serde_json::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Api(ref err) => write!(f, "API error: {}", err),
            CliError::Json(ref err) => write!(f, "JSON error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Api(ref err) => err.description(),
            CliError::Json(ref err) => error::Error::description(err),
            CliError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Api(ref err) => Some(err),
            CliError::Json(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
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
