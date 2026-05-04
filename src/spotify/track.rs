use std::{error::Error, fmt::Display};

use chrono::NaiveDate;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::spotify::{
    auth::Auth,
    utils::{self, deserialize_date},
};

pub async fn get_track(id: &str, auth: &Auth) -> Result<Track, Box<dyn Error>> {
    let base = Url::parse("https://api.spotify.com/v1/tracks/")?;
    let href = base.join(id)?;
    utils::fetch_model::<Track, _>(href, auth).await
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackAlbum {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub release_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackMeta {
    pub upc: Option<String>,
    pub isrc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub duration_ms: u32,
    pub album: TrackAlbum,
    pub artists: Vec<Artist>,
    pub external_ids: TrackMeta,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let artist_names = self
            .artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<&str>>()
            .join(" and ");
        write!(
            f,
            "{} by {} on {}",
            self.name, artist_names, self.album.name
        )
    }
}
