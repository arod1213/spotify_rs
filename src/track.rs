use std::fmt::Display;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{album::Album, utils::deserialize_date};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackAlbum {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub release_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackMeta {
    pub upc: Option<String>,
    pub isrc: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackDetail {
    pub id: String,
    pub name: String,
    pub duration_ms: u32,
    pub album: Album,
    pub artists: Vec<Artist>,
    pub external_ids: TrackMeta,
}

impl TrackDetail {
    pub fn from_track(track: Track, album: Album) -> Self {
        Self {
            id: track.id,
            name: track.name,
            duration_ms: track.duration_ms,
            album,
            artists: track.artists,
            external_ids: track.external_ids,
        }
    }
}
