use std::error::Error;

use crate::spotify::{
    album::{self, Album},
    auth::{self, Auth},
    search::{self, SearchInput},
    track::{self, Track},
};

pub mod spotify;

pub struct Spotify {
    pub auth: Auth,
}
impl Spotify {
    pub async fn new(client_id: &str, client_secret: &str) -> Result<Self, Box<dyn Error>> {
        let auth = auth::get_token(client_id, client_secret).await?;
        Ok(Self { auth })
    }

    pub async fn search(
        &self,
        q: Vec<SearchInput>,
        limit: usize,
    ) -> Result<Vec<Track>, Box<dyn Error>> {
        search::find_track(&self.auth, q, limit).await
    }

    pub async fn album(&self, id: &str) -> Result<Album, Box<dyn Error>> {
        album::get_album(id, &self.auth).await
    }

    pub async fn track(&self, id: &str) -> Result<Track, Box<dyn Error>> {
        track::get_track(id, &self.auth).await
    }
}
