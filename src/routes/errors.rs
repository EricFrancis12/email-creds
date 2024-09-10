use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    error: String,
}

impl APIError {
    pub fn new(error: String) -> Self {
        APIError { error }
    }

    pub fn from_err(err: Error) -> Self {
        APIError::new(err.to_string())
    }
}
