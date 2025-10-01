pub mod auth;
pub mod config;
pub mod parser;

use crate::track::{Track, release_year};

use anyhow::Result;
use futures::StreamExt;

use rspotify::model::PlaylistItem;
use rspotify::{
    AuthCodePkceSpotify,
    model::{Market, PlaylistId},
    prelude::BaseClient,
};
use rspotify::{ClientError, model};

pub struct SpotifyPlaylistClient {
    client: AuthCodePkceSpotify,
}

impl SpotifyPlaylistClient {
    pub fn new(client: AuthCodePkceSpotify) -> Self {
        Self { client }
    }

    pub async fn get_tracks(&self, playlist_id: &str) -> Result<Vec<Track>> {
        let pid = PlaylistId::from_id_or_uri(playlist_id)?;
        let mut result = Vec::new();

        let playlist = self
            .client
            .playlist_items(pid.clone(), None, Some(Market::FromToken));

        let items: Vec<Result<PlaylistItem, ClientError>> = playlist.collect().await;
        result.extend(
            items
                .into_iter()
                .filter_map(Result::ok)
                .filter_map(Track::from_item),
        );

        Ok(result)
    }
}

impl Track {
    pub fn from_item(it: PlaylistItem) -> Option<Track> {
        match it.track {
            Some(model::PlayableItem::Track(t)) => {
                let album = t.album;
                let artist_name = t
                    .artists
                    .first()
                    .map(|a| a.name.clone())
                    .unwrap_or_default();

                let dur_ms = t.duration.num_milliseconds();

                let year = release_year(album.release_date);

                Some(Track::new(
                    t.name,
                    album.name,
                    artist_name,
                    Some(t.track_number as u32),
                    year,
                    Some(dur_ms as u128),
                ))
            }
            _ => None,
        }
    }
}
