use std::io::Error;

use common::{Track, TrackListProvider};

pub struct LocalMusicParser {}

impl LocalMusicParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TrackListProvider for LocalMusicParser {
    type Error = Error;

    fn get_tracks(&self) -> Result<Vec<Track>, Self::Error> {
        unimplemented!("LocalMusicParser get_tracks is unimplemented");
    }
}
