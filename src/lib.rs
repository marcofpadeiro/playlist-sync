pub mod config;
pub mod local_music_library_parser;
pub mod provider;
pub mod spotify_playlist_parser;
pub mod track;

use std::collections::HashMap;

use anyhow::Result;

use crate::{
    provider::PlaylistProvider, spotify_playlist_parser::parser::SpotifyParser, track::Track,
};

pub async fn get_tracks_from_playlist(playlist_url: &str) -> Result<Vec<Track>> {
    let parsed_id = playlist_url.split('?').next().unwrap().split('/').last();

    let id = parsed_id.ok_or_else(|| anyhow::anyhow!("Invalid playlist URL"))?;

    SpotifyParser::get_tracks_from_playlist(id.to_string()).await
}

pub async fn get_tracks_from_local(path: &str) -> Result<HashMap<String, Vec<Track>>> {
    local_music_library_parser::get_local_music_library_tracks(path)
}
