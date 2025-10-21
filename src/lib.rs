pub mod local_music_library_parser;
pub mod provider;
pub mod spotify;
pub mod track;

use std::collections::HashMap;

use anyhow::Result;

use crate::{
    local_music_library_parser::LocalTrack, provider::PlaylistProvider,
    spotify::provider::SpotifyProvider, track::Track,
};

pub async fn get_tracks_from_playlist(playlist_url: String) -> Result<Vec<Track>> {
    let provider = SpotifyProvider::new_from_env().await?;
    provider.get_tracks_from_playlist(playlist_url).await
}

pub async fn get_tracks_from_local(path: &str) -> Result<HashMap<String, Vec<LocalTrack>>> {
    local_music_library_parser::get_local_music_library_tracks(path)
}
