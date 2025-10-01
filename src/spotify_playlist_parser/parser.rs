use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use rspotify::AuthCodePkceSpotify;

use crate::{
    config::{cache_file_path, load_spotify_creds},
    provider::PlaylistProvider,
    spotify_playlist_parser::{
        SpotifyPlaylistClient,
        auth::SpotifyAuth,
        config::{load_token, save_token},
    },
    track::Track,
};

pub struct SpotifyParser {}

#[async_trait]
impl PlaylistProvider for SpotifyParser {
    async fn get_tracks_from_playlist(playlist_url_or_id: String) -> Result<Vec<Track>> {
        let parsed_id = playlist_url_or_id
            .split('?')
            .next()
            .unwrap()
            .split('/')
            .last();

        let id = parsed_id.ok_or_else(|| anyhow::anyhow!("Invalid playlist URL"))?;

        let creds = load_spotify_creds()?;
        let cache_path = cache_file_path();

        let auth = SpotifyAuth::new(creds.client_id, creds.redirect_uri);
        let mut client: AuthCodePkceSpotify = auth.build_client();
        client.config.token_refreshing = true;

        let system_time = SystemTime::now();
        let current_time_seconds = system_time
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_secs())
            .unwrap_or(0);

        if let Ok(tok) = load_token(&cache_path) {
            if let Some(expiration_time) = tok.expires_at {
                let expiration_time_seconds = expiration_time.timestamp() as u64;
                if expiration_time_seconds <= current_time_seconds {
                    println!("Token has expired.");
                    auth_when_token_not_valid(auth, client.clone(), cache_path).await?;
                } else {
                    println!("Token is still valid.");
                    let mut guard = client
                        .token
                        .lock()
                        .await
                        .map_err(|_| anyhow!("token mutex poisoned"))?;
                    *guard = Some(tok);
                }
            }
        } else {
            auth_when_token_not_valid(auth, client.clone(), cache_path).await?;
        }

        let sp = SpotifyPlaylistClient::new(client);
        let tracks = sp.get_tracks(&id).await?;

        Ok(tracks)
    }
}

async fn auth_when_token_not_valid(
    auth: SpotifyAuth,
    client: AuthCodePkceSpotify,
    cache_path: PathBuf,
) -> Result<()> {
    let authed = auth.authenticate().await?;

    let tok = authed
        .token
        .lock()
        .await
        .map_err(|_| anyhow!("token mutex poisoned"))?
        .clone()
        .ok_or_else(|| anyhow!("no token after OAuth"))?;

    save_token(&cache_path, &tok)?;

    let mut guard = client
        .token
        .lock()
        .await
        .map_err(|_| anyhow!("token mutex poisoned"))?;
    *guard = Some(tok);

    Ok(())
}
