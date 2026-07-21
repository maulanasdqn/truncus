pub mod ingest;
pub mod lessons;
pub mod notes;
pub mod read;

use std::collections::HashMap;
use worker::{Request, Result};

pub fn query_params(req: &Request) -> Result<HashMap<String, String>> {
    Ok(req
        .url()?
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect())
}
