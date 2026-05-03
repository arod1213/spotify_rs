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

pub async fn find_track(
    auth: &Auth,
    track_name: &str,
    artist_name: &str,
) -> Result<Vec<Track>, Box<dyn Error>> {
    let mut href = Url::parse("https://api.spotify.com/v1/search")?;
    let query = format!("track:{} artist:{}", track_name, artist_name);
    let _ = href
        .query_pairs_mut()
        .append_pair("q", &query)
        .append_pair("type", "track")
        .append_pair("limit", "5");
    let results = fetch_model::<SearchResults, _>(href, auth).await?;
    Ok(results.tracks.items)
}
