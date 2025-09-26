use std::io::Error;

use crate::{provider::PlaylistProvider, track::Track};

pub struct SpotifyPlaylistParser {}

impl SpotifyPlaylistParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlaylistProvider for SpotifyPlaylistParser {
    type Error = Error;

    fn get_tracks(&self) -> Result<Vec<Track>, Self::Error> {
        unimplemented!("SpotifyPlaylistParser get_tracks is unimplemented");
    }
}
