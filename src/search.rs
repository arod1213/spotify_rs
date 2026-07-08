use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::track::Track;

#[derive(Deserialize)]
pub struct Items {
    pub items: Vec<Track>,
}
#[derive(Deserialize)]
pub struct SearchResults {
    pub tracks: Items,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchInput {
    Isrc(String),
    Track(String),
    Artist(String),
    Album(String),
    Upc(String),
    Year(u32),
}

impl SearchInput {
    pub fn as_query_fragment(&self) -> String {
        match self {
            SearchInput::Isrc(x) => format!("isrc:{x}"),
            SearchInput::Track(x) => format!("track:{x}"),
            SearchInput::Artist(x) => format!("artist:{x}"),
            SearchInput::Album(x) => format!("album:{x}"),
            SearchInput::Upc(x) => format!("upc:{x}"),
            SearchInput::Year(x) => format!("year:{x}"),
        }
    }
}
