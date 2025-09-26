pub mod config;
pub mod local_music_library_parser;
pub mod provider;
pub mod spotify_playlist_parser;
pub mod track;

use anyhow::Result;

use crate::{
    provider::PlaylistProvider, spotify_playlist_parser::parser::SpotifyParser, track::Track,
};

pub async fn get_tracks_from_playlist(playlist_url: &str) -> Result<Vec<Track>> {
    let parsed_id = playlist_url.split('?').next().unwrap().split('/').last();

    let id = parsed_id.ok_or_else(|| anyhow::anyhow!("Invalid Spotify playlist URL"))?;

    SpotifyParser::get_tracks_from_playlist(id.to_string()).await
}
