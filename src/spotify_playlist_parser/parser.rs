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
    async fn get_tracks_from_playlist(playlist_url: String) -> Result<Vec<Track>> {
        let creds = load_spotify_creds()?;
        let cache_path = cache_file_path();

        let auth = SpotifyAuth::new(creds.client_id, creds.redirect_uri);
        let mut client: AuthCodePkceSpotify = auth.build_client();
        client.config.token_refreshing = true;

        if let Ok(tok) = load_token(&cache_path) {
            let mut guard = client
                .token
                .lock()
                .await
                .map_err(|_| anyhow!("token mutex poisoned"))?;
            *guard = Some(tok);
        } else {
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
        }

        let sp = SpotifyPlaylistClient::new(client);
        let tracks = sp.get_tracks(&playlist_url).await?;

        Ok(tracks)
    }
}
