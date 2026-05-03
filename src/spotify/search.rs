use std::error::Error;

use reqwest::Url;
use serde::Deserialize;

use crate::spotify::{auth::Auth, track::Track, utils::fetch_model};

#[derive(Deserialize)]
pub struct Items {
    pub items: Vec<Track>,
}
#[derive(Deserialize)]
pub struct SearchResults {
    pub tracks: Items,
}

pub async fn find_track(auth: &Auth, q: Vec<SearchInput>) -> Result<Vec<Track>, Box<dyn Error>> {
    let mut href = Url::parse("https://api.spotify.com/v1/search")?;

    let query_text = q
        .iter()
        .map(SearchInput::as_query_fragment)
        .collect::<Vec<_>>()
        .join(" ");
    {
        let mut query = href.query_pairs_mut();
        query.append_pair("q", &query_text);
        query.append_pair("type", "track");
        query.append_pair("limit", "5");
    }

    let results = fetch_model::<SearchResults, _>(href, auth).await?;
    Ok(results.tracks.items)
}

pub enum SearchInput {
    Isrc(String),
    Track(String),
    Artist(String),
    Album(String),
    Upc(String),
    Year(u32),
}
impl SearchInput {
    fn as_query_fragment(&self) -> String {
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
