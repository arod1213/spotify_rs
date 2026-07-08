use std::error::Error;

use reqwest::{Client, Url};

use crate::{
    album::{Album, AlbumTrack, AlbumTracks},
    artist::{ArtistAlbum, ArtistAlbumRes, IncludeGroup},
    auth::Auth,
    search::{SearchInput, SearchResults},
    track::{Track, TrackDetail},
    utils::fetch_model,
};

pub mod album;
pub mod artist;
mod auth;
pub mod search;
pub mod track;
mod utils;

pub struct Spotify {
    auth: Auth,
    client: Client,
}
impl Spotify {
    pub async fn new(client_id: &str, client_secret: &str) -> Result<Self, Box<dyn Error>> {
        let auth = auth::get_token(client_id, client_secret).await?;
        Ok(Self {
            auth,
            client: Client::new(),
        })
    }

    /// limit: max of 10
    pub async fn get_artist_albums(
        &self,
        artist_id: &str,
        include_groups: &[IncludeGroup],
        limit: usize,
    ) -> Result<Vec<ArtistAlbum>, Box<dyn Error>> {
        let mut href = Url::parse(&format!(
            "https://api.spotify.com/v1/artists/{artist_id}/albums"
        ))?;
        let include_str = include_groups
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        {
            let mut query = href.query_pairs_mut();
            query.append_pair("include_groups", &include_str);
            query.append_pair("limit", &limit.to_string());
        }
        let results =
            fetch_model::<ArtistAlbumRes, _>(href, &self.auth, self.client.clone()).await?;
        Ok(results.items)
    }

    pub async fn album_tracks(
        &self,
        album_id: &str,
        limit: usize,
    ) -> Result<Vec<AlbumTrack>, Box<dyn Error>> {
        let mut href = Url::parse(&format!(
            "https://api.spotify.com/v1/albums/{album_id}/tracks"
        ))?;
        {
            let mut query = href.query_pairs_mut();
            query.append_pair("limit", &limit.to_string());
        }
        let results = fetch_model::<AlbumTracks, _>(href, &self.auth, self.client.clone()).await?;
        Ok(results.items)
    }

    pub async fn artist_tracks(
        &self,
        artist_id: &str,
        include_groups: &[IncludeGroup],
    ) -> Result<Vec<TrackDetail>, Box<dyn Error>> {
        let mut all_tracks = vec![];

        let artist_albums = self
            .get_artist_albums(artist_id, include_groups, 10)
            .await?;
        for artist_album in artist_albums {
            let album = self.album(&artist_album.id).await?;
            let album_tracks = self.album_tracks(&artist_album.id, 20).await?;
            for album_track in album_tracks {
                let track = self.track(&album_track.id).await?;
                let track_detail = TrackDetail::from_track(track, album.clone());
                all_tracks.push(track_detail);
            }
        }
        Ok(all_tracks)
    }

    pub async fn search(
        &self,
        q: Vec<SearchInput>,
        limit: usize,
    ) -> Result<Vec<Track>, Box<dyn Error>> {
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
            query.append_pair("limit", &limit.to_string());
        }
        let results =
            fetch_model::<SearchResults, _>(href, &self.auth, self.client.clone()).await?;
        Ok(results.tracks.items)
    }

    pub async fn album(&self, id: &str) -> Result<Album, Box<dyn Error>> {
        let base = Url::parse("https://api.spotify.com/v1/albums/")?;
        let href = base.join(id)?;
        utils::fetch_model::<Album, _>(href, &self.auth, self.client.clone()).await
    }

    pub async fn track(&self, id: &str) -> Result<Track, Box<dyn Error>> {
        let base = Url::parse("https://api.spotify.com/v1/tracks/")?;
        let href = base.join(id)?;
        utils::fetch_model::<Track, _>(href, &self.auth, self.client.clone()).await
    }
}
