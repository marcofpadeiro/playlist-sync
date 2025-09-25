use std::io::Error;

use common::{Track, TrackListProvider};

pub struct SpotifyPlaylistParser {}

impl SpotifyPlaylistParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TrackListProvider for SpotifyPlaylistParser {
    type Error = Error;

    fn get_tracks(&self) -> Result<Vec<Track>, Self::Error> {
        unimplemented!("SpotifyPlaylistParser get_tracks is unimplemented");
    }
}
