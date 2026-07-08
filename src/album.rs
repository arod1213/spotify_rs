use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{track::Artist, utils::deserialize_date};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumImage {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumMeta {
    pub upc: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub label: String,

    #[serde(default)]
    pub images: Vec<AlbumImage>,

    #[serde(deserialize_with = "deserialize_date")]
    pub release_date: NaiveDate,
    pub external_ids: AlbumMeta,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumTracks {
    pub items: Vec<AlbumTrack>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumTrack {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
}
