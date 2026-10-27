use serde::{Deserialize, Serialize};

use crate::track::TrackMeta;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistRes {
    pub items: Vec<Inner>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Inner {
    pub item: PlaylistItem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistItem {
    pub id: String,
    pub name: String,
    pub external_ids: TrackMeta,
}
