use std::error::Error;

use chrono::NaiveDate;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::spotify::{
    auth::Auth,
    utils::{self, deserialize_date},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AlbumMeta {
    pub upc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub label: String,

    #[serde(deserialize_with = "deserialize_date")]
    pub release_date: NaiveDate,
    pub external_ids: AlbumMeta,
}

pub async fn get_album(id: &str, auth: &Auth) -> Result<Album, Box<dyn Error>> {
    let base = Url::parse("https://api.spotify.com/v1/albums/")?;
    let href = base.join(id)?;
    utils::fetch_model::<Album, _>(href, auth).await
}
