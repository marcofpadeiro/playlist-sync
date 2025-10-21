use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::{StreamExt, future::join, stream};
use rspotify::{
    ClientCredsSpotify, ClientError, ClientResult, Credentials,
    model::{Market, PlaylistId, PlaylistItem},
    prelude::BaseClient,
};

use crate::{
    provider::{PlaylistProvider, Provider},
    track::Track,
};

pub struct SpotifyProvider {
    client: ClientCredsSpotify,
}

#[async_trait]
impl PlaylistProvider for SpotifyProvider {
    fn provider(&self) -> Provider {
        Provider::Spotify
    }
    // make it blazingly fast with manual pagination
    async fn get_tracks_from_playlist(&self, playlist_uri_or_id: &str) -> Result<Vec<Track>> {
        let pid = PlaylistId::from_id_or_uri(&playlist_uri_or_id)
            .with_context(|| format!("Invalid playlist: {playlist_uri_or_id}"))?;

        let client = &self.client;
        let limit = 1;
        let first = client
            .playlist_items_manual(
                pid.clone(),
                None,
                Some(Market::Country(rspotify::model::Country::Portugal)),
                Some(limit),
                None,
            )
            .await
            .context("probe first page")?;
        let total = first.total as usize;
        if total == 0 {
            return Ok(vec![]);
        }

        let limit = 100;
        let pages = (0..total).step_by(limit).collect::<Vec<_>>();

        let concurrency = 6;

        let results: Vec<ClientResult<_>> = stream::iter(pages)
            .map(|offset| {
                let pid = pid.clone();
                async move {
                    client
                        .playlist_items_manual(
                            pid,
                            None,
                            Some(Market::Country(rspotify::model::Country::Portugal)),
                            Some(limit as u32),
                            Some(offset as u32),
                        )
                        .await
                }
            })
            .buffer_unordered(concurrency)
            .collect()
            .await;

        let mut out = Vec::with_capacity(total);

        for page in results {
            let page = page?;
            for item in page.items.into_iter() {
                if let Some(track) = Track::from_item(item) {
                    out.push(track);
                }
            }
        }
        Ok(out)
    }
}

#[allow(dead_code)]
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
