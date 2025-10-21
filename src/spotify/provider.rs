use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use rspotify::{
    ClientCredsSpotify, ClientError, Credentials,
    model::{Market, PlaylistId, PlaylistItem},
    prelude::BaseClient,
};

use crate::{provider::PlaylistProvider, track::Track};

pub struct SpotifyProvider {
    client: ClientCredsSpotify,
}

#[async_trait]
impl PlaylistProvider for SpotifyProvider {
    async fn get_tracks_from_playlist(&self, playlist_uri_or_id: String) -> Result<Vec<Track>> {
        let playlist_id = PlaylistId::from_id_or_uri(&playlist_uri_or_id)
            .with_context(|| format!("Invalid playlist: {playlist_uri_or_id}"))?;

        let playlist = (&self)
            .client
            .playlist_items(playlist_id, None, Some(Market::FromToken));

        let mut result = Vec::new();

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

impl SpotifyProvider {
    pub async fn new_from_env() -> Result<Self> {
        let creds = Credentials::from_env().ok_or_else(|| {
            anyhow::anyhow!("Credentials not found on environment variables or .env file")
        })?;

        let client = ClientCredsSpotify::new(creds);
        client.request_token().await?;

        Ok(SpotifyProvider { client: client })
    }
    pub async fn new(client_id: &str, client_secret: &str) -> Result<Self> {
        let creds = Credentials::new(client_id, client_secret);
        let client = ClientCredsSpotify::new(creds);
        client.request_token().await?;

        Ok(SpotifyProvider { client: client })
    }
}
