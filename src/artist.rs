use std::fmt::Display;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::utils::deserialize_date;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncludeGroup {
    Single,
    AppearsOn,
    Album,
    Compilation,
}
impl Display for IncludeGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            IncludeGroup::Single => "single",
            IncludeGroup::AppearsOn => "appears_on",
            IncludeGroup::Album => "album",
            IncludeGroup::Compilation => "compilation",
        };
        write!(f, "{x}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistAlbumRes {
    pub items: Vec<ArtistAlbum>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistAlbum {
    pub id: String,
    pub name: String,
    pub total_tracks: usize,
    #[serde(deserialize_with = "deserialize_date")]
    pub release_date: NaiveDate,
}
