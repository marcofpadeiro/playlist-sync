use async_trait::async_trait;

use crate::track::Track;

#[async_trait]
pub trait PlaylistProvider {
    async fn get_tracks_from_playlist(playlist_url: String) -> anyhow::Result<Vec<Track>>;
}
