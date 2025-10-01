pub mod config;
pub mod local_music_library_parser;
pub mod provider;
pub mod spotify_playlist_parser;
pub mod track;

use std::collections::HashMap;

use anyhow::Result;

use crate::{
    local_music_library_parser::LocalTrack, provider::PlaylistProvider,
    spotify_playlist_parser::parser::SpotifyParser, track::Track,
};

pub async fn get_tracks_from_playlist(playlist_url: String) -> Result<Vec<Track>> {
    SpotifyParser::get_tracks_from_playlist(playlist_url).await
}

pub async fn get_tracks_from_local(path: &str) -> Result<HashMap<String, Vec<LocalTrack>>> {
    local_music_library_parser::get_local_music_library_tracks(path)
}
